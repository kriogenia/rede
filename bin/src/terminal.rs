use std::sync::OnceLock;

pub static TERM_LOCK: OnceLock<Terminal> = OnceLock::new();

#[macro_export]
macro_rules! standard {
    ($($arg:tt)*) => {{
        TERM_LOCK.get().unwrap().print_standard(format!($($arg)*));
    }}
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => {{
        TERM_LOCK.get().unwrap().print_verbose(format!($($arg)*));
    }}
}

#[derive(Debug)]
pub struct Terminal {
    mode: Mode,
}

impl Terminal {
    pub fn new(quiet: bool, verbose: bool) -> Self {
        let mode = match (quiet, verbose) {
            (true, false) => Mode::Quiet,
            (false, true) => Mode::Verbose,
            (false, false) => Mode::Standard,
            (true, true) => unreachable!("quiet and verbose are mutually exclusive"),
        };
        Self { mode }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn print_standard(&self, msg: impl AsRef<str>) {
        match self.mode {
            Mode::Standard | Mode::Verbose => println!("{}", msg.as_ref()),
            Mode::Quiet => {}
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn print_verbose(&self, msg: impl AsRef<str>) {
        if self.mode == Mode::Verbose {
            println!("{}", msg.as_ref());
        }
    }
}

#[derive(Debug, PartialEq)]
enum Mode {
    Quiet,
    Standard,
    Verbose,
}
