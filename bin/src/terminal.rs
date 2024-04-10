use std::borrow::Cow;
use std::sync::OnceLock;

pub static TERM_LOCK: OnceLock<Terminal> = OnceLock::new();

#[macro_export]
macro_rules! standard {
    ($($arg:tt)*) => {{
        use $crate::terminal::TERM_LOCK;
        TERM_LOCK.get().unwrap().print_standard(format!($($arg)*));
    }}
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => {{
        use $crate::terminal::TERM_LOCK;
        TERM_LOCK.get().unwrap().print_verbose(format!($($arg)*));
    }}
}

#[derive(Debug)]
pub struct Terminal {
    mode: Mode,
    no_color: bool,
}

impl Terminal {
    pub fn new(quiet: bool, verbose: bool, no_color: bool) -> Self {
        let mode = match (quiet, verbose) {
            (true, false) => Mode::Quiet,
            (false, true) => Mode::Verbose,
            (false, false) => Mode::Standard,
            (true, true) => unreachable!("quiet and verbose are mutually exclusive"),
        };
        Self { mode, no_color }
    }

    #[inline]
    pub fn print_standard(&self, msg: impl AsRef<str>) {
        let msg = self.remove_color(msg.as_ref());
        match self.mode {
            Mode::Standard | Mode::Verbose => println!("{msg}"),
            Mode::Quiet => {}
        }
    }

    #[inline]
    pub fn print_verbose(&self, msg: impl AsRef<str>) {
        if self.mode == Mode::Verbose {
            println!("{}", self.remove_color(msg.as_ref()));
        }
    }

    #[inline]
    fn remove_color<'a>(&self, msg: &'a str) -> Cow<'a, str> {
        if self.no_color {
            console::strip_ansi_codes(msg)
        } else {
            msg.into()
        }
    }
}

#[derive(Debug, PartialEq)]
enum Mode {
    Quiet,
    Standard,
    Verbose,
}
