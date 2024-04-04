use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use miette::miette;

pub const STDIN_ARG: &str = "-";

pub fn open_file_or_stdin(filename: &str) -> miette::Result<Box<dyn BufRead>> {
    if filename == STDIN_ARG {
        return Ok(Box::new(BufReader::new(io::stdin())));
    }

    match File::open(filename) {
        Ok(file) => Ok(Box::new(BufReader::new(file))),
        Err(e) => Err(miette!("Unknown file: {e}")),
    }
}
