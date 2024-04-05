use crate::errors::InnerError;
use miette::Result;
use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

pub const STDIN_ARG: &str = "-";

pub fn read_to_string(source: &str) -> Result<String> {
    let (file, mut reader) = open_file_or_stdin(source)?;
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .map_err(|e| InnerError::io(file, e))?;
    Ok(buffer)
}

fn open_file_or_stdin(filename: &str) -> Result<(Cow<str>, Box<dyn BufRead>)> {
    if filename == STDIN_ARG {
        return Ok((filename.into(), Box::new(BufReader::new(io::stdin()))));
    }

    let filename = add_extension(filename);
    let file = File::open(&*filename).map_err(|e| InnerError::io(filename.clone(), e))?;
    Ok((filename, Box::new(BufReader::new(file))))
}

#[inline]
fn add_extension(filename: &str) -> Cow<str> {
    if filename.ends_with(".toml") {
        filename.into()
    } else {
        format!("{filename}.toml").into()
    }
}
