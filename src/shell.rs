use super::error::IpaError;
use std::process::{Command, Output};

pub fn execute(commands: &Vec<String>) -> Result<Output, IpaError> {
    if commands.len() == 0 {
        return Err(IpaError::EmptyCommand);
    }
    println!("Executing command: {:?}", commands);
    Ok(Command::new(&commands[0]).args(&commands[1..]).output()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_commands() {
        let output = execute(&vec![
            String::from("echo"),
            String::from("foo"),
            String::from("bar"),
        ])
        .unwrap();
        assert!(output.status.success());
        assert_eq!("foo bar\n", String::from_utf8_lossy(&output.stdout));
    }

    #[test]
    fn test_single_command() {
        let output = execute(&vec![String::from("ls")]).unwrap();
        assert!(output.status.success());
    }

    #[test]
    fn test_invalid_command() {
        assert!(execute(&vec![String::from("bla")]).is_err());
    }

    #[test]
    fn test_empty_commands() {
        let commands: Vec<String> = vec![];
        assert!(execute(&commands).is_err())
    }
}
