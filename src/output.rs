use std::fmt;
use std::io::{self, Write};

pub fn print(args: fmt::Arguments<'_>) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_fmt(args)?;
    stdout.write_all(b"\n")
}

pub fn eprint(args: fmt::Arguments<'_>) -> io::Result<()> {
    let mut stderr = io::stderr().lock();
    stderr.write_fmt(args)?;
    stderr.write_all(b"\n")
}

pub fn eblank_line() -> io::Result<()> {
    eprint(format_args!(""))
}

pub fn flush() -> io::Result<()> {
    io::stdout().lock().flush()
}

#[deprecated = "use output::eprint instead"]
#[allow(dead_code)]
pub fn writeln(args: fmt::Arguments<'_>) -> io::Result<()> {
    eprint(args)
}

#[deprecated = "use output::print instead"]
#[allow(dead_code)]
pub fn write(args: fmt::Arguments<'_>) -> io::Result<()> {
    print(args)
}

#[deprecated = "use output::eblank_line instead"]
#[allow(dead_code)]
pub fn blank_line() -> io::Result<()> {
    eblank_line()
}
