use crate::errors::ParsingError;
use miette::Result;
use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

pub const STDIN_ARG: &str = "-";

pub fn input_to_string(source: &str) -> Result<String> {
    let (file, mut reader) = open_file_or_stdin(source)?;
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .map_err(|e| ParsingError::io(file, e))?;
    Ok(buffer)
}

fn open_file_or_stdin(filename: &str) -> Result<(Cow<str>, Box<dyn BufRead>)> {
    if filename == STDIN_ARG {
        return Ok((filename.into(), Box::new(BufReader::new(io::stdin()))));
    }

    let filename = add_extension(filename);
    let file = File::open(&*filename).map_err(|e| ParsingError::io(filename.clone(), e))?;
    Ok((filename, Box::new(BufReader::new(file))))
}

#[inline]
fn add_extension(filename: &str) -> Cow<str> {
    if Path::new(filename)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("toml"))
    {
        filename.into()
    } else {
        format!("{filename}.toml").into()
    }
}
