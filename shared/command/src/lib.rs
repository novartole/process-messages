use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Command {
    Capitalize,
    Reverse,
    ToLowerCase,
    ToUpperCase,
}

impl Default for Command {
    fn default() -> Self {
        Command::Capitalize
    }
}

impl Command {
    pub fn call_on(&self, mut input: String) -> String {
        match self {
            Command::Capitalize => {
                if input.chars().nth(0).is_some_and(|c| !c.is_uppercase()) {
                    let (first, rest) = input.split_at_mut(1);

                    let mut capitalized = first.to_uppercase();
                    capitalized.push_str(rest);

                    return capitalized;
                }

                input
            }
            Command::Reverse => {
                let len = input.len();
                input
                    .chars()
                    .into_iter()
                    .rev()
                    .fold(String::with_capacity(len), |mut s, c| {
                        s.push(c);
                        s
                    })
            }
            Command::ToLowerCase => input.to_lowercase(),
            Command::ToUpperCase => input.to_uppercase(),
        }
    }
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Value '{0}' doesn't match any command")]
    ParseFromString(String),
}

impl TryInto<Command> for String {
    type Error = CommandError;

    fn try_into(self) -> Result<Command, Self::Error> {
        let command = match self.as_str() {
            "cap" => Command::Capitalize,
            "rev" => Command::Reverse,
            "lc" => Command::ToLowerCase,
            "uc" => Command::ToUpperCase,
            _ => {
                return Err(CommandError::ParseFromString(self));
            }
        };

        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type DataType<'a> = [&'a str; 5];

    const ACTUAL: &DataType = &["", "1234qQ", "qwerty", "Qwerty", "QwErTy"];

    fn test(command: Command, actual: &DataType) {
        let (expected, command): (DataType, _) = match command {
            Command::Capitalize => (
                ["", "1234qQ", "Qwerty", "Qwerty", "QwErTy"],
                Command::Capitalize,
            ),
            Command::Reverse => (
                ["", "Qq4321", "ytrewq", "ytrewQ", "yTrEwQ"],
                Command::Reverse,
            ),
            Command::ToLowerCase => (
                ["", "1234qq", "qwerty", "qwerty", "qwerty"],
                Command::ToLowerCase,
            ),
            Command::ToUpperCase => (
                ["", "1234QQ", "QWERTY", "QWERTY", "QWERTY"],
                Command::ToUpperCase,
            ),
        };

        actual
            .into_iter()
            .zip(expected)
            .for_each(|(actual, expected)| {
                assert_eq!(command.call_on(actual.to_string()), expected.to_string())
            });
    }

    #[test]
    fn to_upper_case() {
        test(Command::ToUpperCase, ACTUAL);
    }

    #[test]
    fn to_lower_case() {
        test(Command::ToLowerCase, ACTUAL);
    }

    #[test]
    fn reverse() {
        test(Command::Reverse, ACTUAL);
    }

    #[test]
    fn capitalize() {
        test(Command::Capitalize, ACTUAL);
    }
}
