extern crate extract;

use std::env;
use std::io::{stdin, stderr, BufRead};

use extract::regex::Regex;
use extract::errors::*;
use extract::search::*;

fn do_search<T>(re: &Regex, input: T) -> Result<()>
    where T: BufRead
{

    match re.captures_len() {
        2 => {}
        0...1 => return Err(ErrorKind::NothingToExtract.into()),
        _ => return Err(ErrorKind::TooManyCaptures.into()),
    }

    for result in search(&re, input) {
        let result = result?;
        println!("{}", result);
    }
    Ok(())
}

fn run() -> Result<()> {
    if let Some(target) = env::args().nth(1) {
        let re = try!(Regex::new(&target));
        let stdin = stdin();
        let stdin = stdin.lock();
        do_search(&re, stdin)
    } else {
        return Err(ErrorKind::NoMatchingPattern.into());
    }
}

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::io::{BufReader, Empty};

    fn re(txt: &str) -> Regex {
        Regex::new(txt).unwrap()
    }

    fn empty_reader() -> BufReader<Empty> {
        BufReader::new(io::empty())
    }

    #[test]
    fn regex_with_no_capture_is_an_error() {
        let empty_reader = empty_reader();
        match do_search(&re("no capture"), empty_reader) {
            Err(_) => {}
            _ => panic!("Should throw when nothing to capture"),
        }
    }

    #[test]
    fn regex_with_two_captures_is_an_error() {
        let empty_reader = empty_reader();
        match do_search(&re("(first) (second)"), empty_reader) {
            Err(_) => {}
            _ => panic!("Should throw when there's more than one capture group"),
        }
    }

    #[test]
    fn single_regex_is_ok() {
        let empty_reader = empty_reader();
        do_search(&re("(single)"), empty_reader).unwrap();
    }
}
