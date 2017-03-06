
use std::io::BufRead;
use std::io;
use std::fmt::Display;
use regex::Regex;
use errors::*;

pub enum Match {
    Simple(String),
    MultiMatch(Vec<String>),
}

impl Display for Match {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            &Match::Simple(ref s) => write!(fmt, "{}", s),
            _ => unimplemented!(),
        }
    }
}

pub struct MatchIterator<'a, T>
    where T: Iterator<Item = io::Result<String>>
{
    source: T,
    regex: &'a Regex,
}

impl<'a, T> MatchIterator<'a, T>
    where T: Iterator<Item = io::Result<String>>
{
    pub fn all_or_error(self) -> Result<Vec<Match>> {
        let mut result = Vec::new();
        for m in self {
            result.push(m?);
        }
        Ok(result)
    }

    fn try_match(&self, string: &str) -> Option<Result<Match>> {
        if let Some(captures) = self.regex.captures(&string) {
            for capture in captures.iter().skip(1) {
                if let Some(c) = capture {
                    let string = c.as_str().to_owned();
                    return Some(Ok(Match::Simple(string)));
                }
            }
        }

        None
    }
}

impl<'a, T> Iterator for MatchIterator<'a, T>
    where T: Iterator<Item = io::Result<String>>
{
    type Item = Result<Match>;

    fn next(&mut self) -> Option<Result<Match>> {
        while let Some(line) = self.source.next() {
            let r = match line {
                Err(e) => Some(Err(::std::convert::From::from(e))),
                Ok(l) => self.try_match(&l),
            };
            if r.is_some() {
                return r;
            }
        }
        None
    }
}

pub fn search<T>(re: &Regex, from: T) -> MatchIterator<io::Lines<T>>
    where T: BufRead
{

    MatchIterator {
        source: from.lines(),
        regex: re,
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use self::tempdir::TempDir;
    use std::fs::File;
    use std::path::PathBuf;
    use std::io::{Write, BufReader};

    use super::*;

    struct TempLines {
        _td: TempDir,
        file: PathBuf,
    }

    fn lines(from: &str) -> TempLines {
        let dir = TempDir::new("extract-test")
            .expect("Cannot create temporary directory for tests");
        let fpath = dir.path().join("temp_file");
        let mut file = File::create(&fpath).expect("Unable to create temporary file for test");
        file.write(&from.as_bytes()).expect("Unable to write to temporary file in test setup");
        drop(file);


        TempLines {
            _td: dir,
            file: PathBuf::from(fpath),
        }
    }

    impl TempLines {
        fn reader(&self) -> BufReader<File> {
            let file = File::open(&self.file)
                .expect("Unable to open temporary file created for test");
            BufReader::new(file)
        }
    }

    fn matches_for(target: &str, body: &str) -> Vec<Match> {
        let l = lines(&body).reader();
        let re = Regex::new(&target).unwrap();
        search(&re, l).all_or_error().unwrap()
    }

    #[test]
    fn single_unmatched_line_yields_no_results() {
        let result = matches_for("(match)", "nothing");
        assert!(result.is_empty());
    }

    #[test]
    fn single_matched_line_yields_one_result() {
        let result = matches_for("(match)", "match");
        assert!(!result.is_empty());
    }

    #[test]
    fn results_only_for_matched_lines() {
        let result = matches_for("(match)", "match\nnothing\nmatch\nnothing");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn match_contains_text_from_capture_group() {
        let ref result = matches_for(r"(\d+)", "hello42world")[0];
        match result {
            &Match::Simple(ref r) => {
                assert_eq!(r, "42");
            }
            _ => {
                panic!();
            }
        }
    }
}
