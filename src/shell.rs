use crate::error::IpaError;
use log::debug;
use serde::{Deserialize, Serialize};
use std::process::{Command, Output};

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

pub fn execute(shell: &Shell) -> Result<Output, IpaError> {
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
