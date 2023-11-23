use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidCmd,
    Parse(ParseIntError),
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidCmd => write!(f, "Invalid command"),
            ParseError::Parse(err) => write!(f, "Failed to parse command parameter: {}", err),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::Parse(err)
    }
}

#[derive(Debug, PartialEq)]
pub enum Cmd {
    Up(usize),
    Down(usize),
    Forward(usize),
}

impl FromStr for Cmd {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Cmd, ParseError> {
        let (dir, amount) = s.split_once(" ").ok_or(ParseError::InvalidCmd)?;
        let amount = amount.parse()?;

        match dir {
            "up" => Ok(Cmd::Up(amount)),
            "down" => Ok(Cmd::Down(amount)),
            "forward" => Ok(Cmd::Forward(amount)),
            _ => Err(ParseError::InvalidCmd),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Cmd>, ParseError> {
    input.split("\n").map(|n| n.parse()).collect()
}

#[aoc(day2, part1)]
pub fn part1(commands: &[Cmd]) -> usize {
    let mut h = 0;
    let mut d = 0;

    for c in commands {
        match c {
            Cmd::Up(x) => d -= x,
            Cmd::Down(x) => d += x,
            Cmd::Forward(x) => h += x,
        }
    }

    h * d
}

#[aoc(day2, part2)]
pub fn part2(commands: &[Cmd]) -> usize {
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;

    for c in commands {
        match c {
            Cmd::Up(x) => aim -= x,
            Cmd::Down(x) => aim += x,
            Cmd::Forward(x) => {
                h += x;
                d += aim * x;
            }
        }
    }

    h * d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let input = input_generator("up 3\ndown 5\nforward 1").expect("Input should be valid");

        assert_eq!(input[0], Cmd::Up(3));
        assert_eq!(input[1], Cmd::Down(5));
        assert_eq!(input[2], Cmd::Forward(1));
    }

    #[test]
    fn foo_command_is_invalid() {
        let input = input_generator("foo 3\ndown 5\nforward 1");

        assert_eq!(input, Err(ParseError::InvalidCmd));
    }

    #[test]
    fn up_without_param_is_invalid() {
        let input = input_generator("up\ndown 5\nforward 1");

        assert_eq!(input, Err(ParseError::InvalidCmd));
    }

    #[test]
    fn empty_input_is_invalid() {
        let input = input_generator("");

        assert_eq!(input, Err(ParseError::InvalidCmd));
    }

    #[test]
    fn invalid_command_format() {
        let input = input_generator("");

        assert_eq!(format!("{}", input.unwrap_err()), "Invalid command");
    }

    #[test]
    fn up_with_letter_is_parse_error() {
        let input = input_generator("up x\ndown 5\nforward 1");

        assert!(matches!(input, Err(ParseError::Parse(_))));
    }

    #[test]
    fn parse_error_format() {
        let input = input_generator("up x\ndown 5\nforward 1");

        assert_eq!(
            format!("{}", input.unwrap_err()),
            "Failed to parse command parameter: invalid digit found in string"
        );
    }

    #[test]
    fn sample1() {
        let commands = vec![
            Cmd::Forward(5),
            Cmd::Down(5),
            Cmd::Forward(8),
            Cmd::Up(3),
            Cmd::Down(8),
            Cmd::Forward(2),
        ];

        assert_eq!(part1(&commands), 150);
    }

    #[test]
    fn sample2() {
        let commands = vec![
            Cmd::Forward(5),
            Cmd::Down(5),
            Cmd::Forward(8),
            Cmd::Up(3),
            Cmd::Down(8),
            Cmd::Forward(2),
        ];

        assert_eq!(part2(&commands), 900);
    }
}
