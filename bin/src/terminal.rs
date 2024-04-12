use std::sync::OnceLock;

pub static TERM_LOCK: OnceLock<Terminal> = OnceLock::new();

#[macro_export]
macro_rules! standard {
    (below[$mode:ident] $($arg:tt)*) => {{
        use $crate::terminal::{TERM_LOCK, Mode};
        TERM_LOCK.get().unwrap().print_between(Mode::Standard, Mode::$mode, format!($($arg)*));
    }};
    ($($arg:tt)*) => {
        use $crate::terminal::{TERM_LOCK, Mode};
        TERM_LOCK.get().unwrap().print_above(Mode::Standard, format!($($arg)*));{
    }};
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => {{
        use $crate::terminal::{TERM_LOCK, Mode};
        TERM_LOCK.get().unwrap().print_above(Mode::Verbose, format!($($arg)*));
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

    #[inline]
    pub fn print_above(&self, mode: Mode, msg: impl AsRef<str>) {
        if self.mode >= mode {
            print(msg);
        }
    }

    #[inline]
    pub fn print_between(&self, from: Mode, until: Mode, msg: impl AsRef<str>) {
        if self.mode >= from && self.mode < until {
            print(msg);
        }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
pub(crate) enum Mode {
    Quiet = 0,
    Standard = 2,
    Verbose = 3,
}

#[inline]
pub fn print(msg: impl AsRef<str>) {
    println!("{}", msg.as_ref());
}
