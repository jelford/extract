#![recursion_limit = "1024"]

pub extern crate regex;

#[macro_use]
extern crate error_chain;
pub mod errors;

pub mod search;
