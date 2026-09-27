//! Every write to stdout goes through `out!` and `outln!`, so a reader that closes the pipe
//! early ends the run quietly instead of with a panic.
//!
//! Rust ignores `SIGPIPE` before `main`, so a write to a closed pipe returns
//! `ErrorKind::BrokenPipe`, and the standard `print!` macros turn that into a panic whose
//! message lands on stderr of whoever piped the output into `head`. A closed stdout means the
//! run could not deliver its output, which is the ladder's exit 2 per
//! `design@thaum@exit-code-ladder`; it is never a verdict, so neither 0 nor 1 may be reported
//! for it. Nothing is printed to stderr for it either: the reader closed the pipe on purpose.
//!
//! The `SIGPIPE` default disposition is not restored instead, because that would also kill the
//! process on a write to a git child that exited early, where `git.rs` reads the error and
//! reports git's own message.

use std::io::{ErrorKind, Write};

/// Writes formatted output to stdout, ending the process with exit 2 on a broken pipe.
pub fn write(args: std::fmt::Arguments<'_>) {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    if let Err(e) = lock.write_fmt(args) {
        if e.kind() == ErrorKind::BrokenPipe {
            std::process::exit(2);
        }
        eprintln!("error: cannot write to stdout: {e}");
        std::process::exit(2);
    }
}

macro_rules! out {
    ($($arg:tt)*) => { $crate::cli::output::write(format_args!($($arg)*)) };
}

macro_rules! outln {
    () => { $crate::cli::output::write(format_args!("\n")) };
    ($($arg:tt)*) => { $crate::cli::output::write(format_args!("{}\n", format_args!($($arg)*))) };
}

pub(crate) use {out, outln};
