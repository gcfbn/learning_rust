use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::{self, Write};

/// Writes messages using colors the same as in `anyhow` crate
pub fn write_colored_error_message(msg: &str) -> io::Result<()> {
    let choice = if atty::is(atty::Stream::Stderr) {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    let mut out = StandardStream::stderr(choice);
    out.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    write!(&mut out, "error: ")?;
    out.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(&mut out, "{}", msg)
}