use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub(crate) fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) struct ErrorMsg {
    pub(crate) wrapped: String
}

impl From<io::Error> for ErrorMsg {
    fn from(err: io::Error) -> Self {
       ErrorMsg { wrapped: format!("IO error: {}", err.to_string()) }
    }
}