use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::path::Path;
use regex::Regex;

pub(crate) fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) struct ErrorMsg {
    pub(crate) wrapped: String
}

impl ErrorMsg {
    pub(crate) fn print(result: Result<(), ErrorMsg>) -> () {
        match result {
            Ok(val) => val,
            Err(err) => println!("Error: {}", err.wrapped)
        }
    }
    pub(crate) fn new(string: &str) -> ErrorMsg {
        ErrorMsg { wrapped: string.to_string() }
    }
}

impl From<io::Error> for ErrorMsg {
    fn from(err: io::Error) -> Self {
       ErrorMsg { wrapped: format!("IO error: {}", err.to_string()) }
    }
}
impl From<ParseIntError> for ErrorMsg {
    fn from(err: ParseIntError) -> Self {
       ErrorMsg { wrapped: format!("ParseIntError: {}", err.to_string()) }
    }
}
impl From<regex::Error> for ErrorMsg {
    fn from(err: regex::Error) -> Self {
       ErrorMsg { wrapped: format!("Failed to compile regex: {}", err.to_string()) }
    }
}