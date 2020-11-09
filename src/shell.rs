use log::debug;
use serde::{Deserialize, Serialize};
use std::io;
use std::process::{Command, Output};

#[derive(Debug)]
pub enum Error {
    /// io error to execute command.
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Self {
        Error::Io(val)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Shell {
    #[serde(default)]
    pub command: String,
}

impl Shell {
    pub fn new(command: &str) -> Self {
        Shell {
            command: command.to_string(),
        }
    }
}

pub fn execute(shell: &Shell) -> Result<Output, Error> {
    debug!("Executing command: {}", shell.command);
    Ok(Command::new("bash")
        .arg("-c")
        .arg(&shell.command)
        .output()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_commands() {
        let output = execute(&Shell::new("echo foo bar")).unwrap();
        assert!(output.status.success());
        assert_eq!("foo bar\n", String::from_utf8_lossy(&output.stdout));
    }

    #[test]
    fn test_single_command() {
        let output = execute(&Shell::new("ls")).unwrap();
        assert!(output.status.success());
    }

    #[test]
    fn test_invalid_command() {
        assert_eq!(false, execute(&Shell::new("bla")).unwrap().status.success());
    }
}
