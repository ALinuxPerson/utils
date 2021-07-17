use std::fmt::Display;
use std::io::{Write, Stdout, Stderr};
use crossterm::style::{Color, Stylize, StyledContent, ContentStyle};
use std::io;
use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

const DIVIDER: char = '┃';

static STDOUT: Lazy<Mutex<Logger<Stdout>>> = Lazy::new(|| Mutex::new(Logger::new(io::stdout())));
static STDERR: Lazy<Mutex<Logger<Stderr>>> = Lazy::new(|| Mutex::new(Logger::new(io::stderr())));

pub struct Logger<W: Write> {
    writer: W
}

impl Logger<Stdout> {
    pub fn stdout() -> MutexGuard<'static, Self> {
        STDOUT.lock().expect("failed to lock stdout")
    }
}

impl Logger<Stderr> {
    pub fn stderr() -> MutexGuard<'static, Self> {
        STDERR.lock().expect("failed to lock stderr")
    }
}

impl<W: Write> Logger<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    fn level(&mut self, message: impl Display, color: Color) -> io::Result<()> {
        writeln!(
            &mut self.writer,
            "{}{}{} {}",
            crossterm::style::SetForegroundColor(color),
            DIVIDER,
            crossterm::style::ResetColor,
            message,
        )
    }

    fn bold_level(&mut self, message: impl Display, color: Color) -> io::Result<()> {
        writeln!(
            &mut self.writer,
            "{}{}{} {}",
            crossterm::style::SetForegroundColor(color),
            DIVIDER,
            crossterm::style::ResetColor,
            message.to_string().bold(),
        )
    }

    pub fn info(&mut self, message: impl Display) -> io::Result<()> {
        self.level(message, Color::Blue)
    }

    pub fn note(&mut self, message: impl Display) -> io::Result<()> {
        self.bold_level(message, Color::Blue)
    }

    pub fn warning(&mut self, message: impl Display) -> io::Result<()> {
        self.level(message, Color::Yellow)
    }

    pub fn error(&mut self, message: impl Display) -> io::Result<()> {
        self.level(message, Color::Red)
    }

    pub fn fatal(&mut self, message: impl Display) -> io::Result<()> {
        self.bold_level(message, Color::Red)
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        ::utils::Logger::stderr().info(format!($($arg)*)).expect("failed to print to stderr")
    };
}

#[macro_export]
macro_rules! oinfo {
    ($($arg:tt)*) => {
        ::utils::Logger::stdout().info(format!($($arg)*)).expect("failed to print to stdout")
    };
}

#[macro_export]
macro_rules! note {
    ($($arg:tt)*) => {
        ::utils::Logger::stderr().note(format!($($arg)*)).expect("failed to print to stderr")
    };
}

#[macro_export]
macro_rules! onote {
    ($($arg:tt)*) => {
        ::utils::Logger::stdout().note(format!($($arg)*)).expect("failed to print to stdout")
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        ::utils::Logger::stderr().warning(format!($($arg)*)).expect("failed to print to stderr")
    };
}

#[macro_export]
macro_rules! owarning {
    ($($arg:tt)*) => {
        ::utils::Logger::stdout().warning(format!($($arg)*)).expect("failed to print to stdout")
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        ::utils::Logger::stderr().error(format!($($arg)*)).expect("failed to print to stderr")
    };
}

#[macro_export]
macro_rules! oerror {
    ($($arg:tt)*) => {
        ::utils::Logger::stdout().error(format!($($arg)*)).expect("failed to print to stdout")
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        ::utils::Logger::stderr().fatal(format!($($arg)*)).expect("failed to print to stderr")
    };
}

#[macro_export]
macro_rules! ofatal {
    ($($arg:tt)*) => {
        ::utils::Logger::stdout().fatal(format!($($arg)*)).expect("failed to print to stdout")
    };
}
