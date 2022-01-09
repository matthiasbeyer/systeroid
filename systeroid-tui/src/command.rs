use std::str::FromStr;
use termion::event::Key;

/// Possible application commands.
#[derive(Debug)]
pub enum Command {
    /// Scroll up on the widget.
    ScrollUp,
    /// Scroll down on the widget.
    ScrollDown,
    /// Process the input.
    ProcessInput,
    /// Update the input buffer.
    UpdateInput(char),
    /// Clear the input buffer.
    ClearInput(bool),
    /// Refresh the application.
    Refresh,
    /// Exit the application.
    Exit,
    /// Do nothing.
    None,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Command::ScrollUp),
            "down" => Ok(Command::ScrollDown),
            "refresh" => Ok(Command::Refresh),
            "exit" | "quit" | "q" | "q!" => Ok(Command::Exit),
            _ => Err(()),
        }
    }
}

impl Command {
    /// Parses a command from the given key.
    pub fn parse(key: Key, input_mode: bool) -> Self {
        if input_mode {
            match key {
                Key::Char('\n') => Command::ProcessInput,
                Key::Char(c) => Command::UpdateInput(c),
                Key::Backspace => Command::ClearInput(false),
                Key::Esc => Command::ClearInput(true),
                _ => Command::None,
            }
        } else {
            match key {
                Key::Up => Command::ScrollUp,
                Key::Down => Command::ScrollDown,
                Key::Char(':') => Command::UpdateInput(' '),
                Key::Char('r') => Command::Refresh,
                Key::Esc => Command::Exit,
                _ => Command::None,
            }
        }
    }
}