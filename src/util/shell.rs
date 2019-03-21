use std::fmt::Display;
use std::io::Write;
use termcolor::Color::{Cyan, Red, Yellow};
use termcolor::{self, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Shell {
    stderr: ShellWriter,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            stderr: ShellWriter::new(),
        }
    }

    pub fn info<T>(&mut self, message: T) -> Result<(), failure::Error>
    where
        T: Display,
    {
        self.stderr.println(&"info:", &message, Cyan)
    }

    pub fn warn<T>(&mut self, message: T) -> Result<(), failure::Error>
    where
        T: Display,
    {
        self.stderr.println(&"warning:", &message, Yellow)
    }

    pub fn error<T>(&mut self, message: T) -> Result<(), failure::Error>
    where
        T: Display,
    {
        self.stderr.println(&"error:", &message, Red)
    }

    pub fn print<T>(&mut self, message: T) -> Result<(), failure::Error>
    where
        T: Display,
    {
        self.stderr.writeln(&message)
    }

    pub fn exit(&mut self, code: i32) -> ! {
        std::process::exit(code)
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

        self.stderr.stream = StandardStream::stderr(choice);

        Ok(())
    }
}

pub struct ShellWriter {
    stream: StandardStream,
}

impl ShellWriter {
    fn new() -> Self {
        Self {
            stream: StandardStream::stderr(ColorChoice::Auto),
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
