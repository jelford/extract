
use regex;
use std::io;

error_chain! {
    foreign_links {
        BadRegex(regex::Error);
        Io(io::Error);
    }

    errors {
        TooManyCaptures {
            description("Too many capture groups; what will I print?")
            display("Too many capture groups; what will I print?")
        }
        NothingToExtract {
            description("No capture group give in regex - nothing to extract")
            display("No capture group give in regex - nothing to extract")
        }
        NoMatchingPattern {
            description("No matching pattern provided")
            display("No matching pattern provided")
        }
    }
}
