use std::fmt::Display;
use std::io::Write;

use termcolor::{
    self, Color, ColorChoice as TermColorChoice, ColorSpec, StandardStream, WriteColor,
};

pub struct Shell {
    stderr: ShellWriter,
    verbosity: Verbosity,
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

impl Shell {
    pub fn new() -> Self {
        Self {
            stderr: ShellWriter::new(),
            verbosity: Verbosity::Normal,
        }
    }

    pub fn info<T>(&mut self, message: T) -> Result<(), failure::Error>
    where
        T: Display,
    {
        match self.verbosity {
            Verbosity::Verbose => self.stderr.println(&"info:", &message, Color::Cyan),
            _ => Ok(()),
        }
    }

    pub fn warn<T>(&mut self, message: T) -> Result<(), failure::Error>
    where
        T: Display,
    {
        match self.verbosity {
            Verbosity::Verbose => self.stderr.println(&"warning:", &message, Color::Yellow),
            _ => Ok(()),
        }
    }

    pub fn error<T>(&mut self, message: T) -> Result<(), failure::Error>
    where
        T: Display,
    {
        match self.verbosity {
            Verbosity::Quiet => Ok(()),
            _ => self.stderr.println(&"error:", &message, Color::Red),
        }
    }

    pub fn print<T>(&mut self, message: T) -> Result<(), failure::Error>
    where
        T: Display,
    {
        match self.verbosity {
            Verbosity::Quiet => Ok(()),
            _ => self.stderr.writeln(&message),
        }
    }

    pub fn exit(&mut self, code: i32) -> ! {
        std::process::exit(code)
    }

    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = verbosity;
    }

    pub fn set_color_choice(&mut self, color: &str) -> Result<(), failure::Error> {
        let choice = match color {
            "never" => ColorChoice::Never,
            "always" => ColorChoice::Always,
            "auto" => ColorChoice::Auto,
            arg => {
                self.error(format!("Invalid color choice: {}", arg))?;
                self.exit(1);
            }
        };

        self.stderr.stream = StandardStream::stderr(choice.into());

        Ok(())
    }
}

pub struct ShellWriter {
    stream: StandardStream,
}

impl ShellWriter {
    fn new() -> Self {
        Self {
            stream: StandardStream::stderr(ColorChoice::Auto.into()),
        }
    }

    fn println(
        &mut self,
        status: &dyn Display,
        message: &dyn Display,
        color: Color,
    ) -> Result<(), failure::Error> {
        self.stream.reset()?;
        self.stream
            .set_color(ColorSpec::new().set_bold(true).set_fg(Some(color)))?;

        self.write(status)?;
        self.stream.reset()?;
        self.write(&" ")?;
        self.writeln(message)?;

        Ok(())
    }

    fn write(&mut self, message: &dyn Display) -> Result<(), failure::Error> {
        write!(self.stream, "{}", message)?;

        Ok(())
    }

    fn writeln(&mut self, message: &dyn Display) -> Result<(), failure::Error> {
        writeln!(self.stream, "{}", message)?;

        Ok(())
    }
}

pub enum Verbosity {
    Verbose,
    Normal,
    Quiet,
}

pub enum ColorChoice {
    Never,
    Always,
    Auto,
}

impl Into<TermColorChoice> for ColorChoice {
    fn into(self) -> TermColorChoice {
        match self {
            ColorChoice::Never => TermColorChoice::Never,
            ColorChoice::Always => TermColorChoice::Always,
            ColorChoice::Auto => {
                if atty::is(atty::Stream::Stderr) {
                    TermColorChoice::Auto
                } else {
                    TermColorChoice::Never
                }
            }
        }
    }
}
