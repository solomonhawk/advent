mod data {
  use std::str::FromStr;

  #[derive(Debug, Clone)]
  pub struct ParseError;

  #[derive(Debug, Clone)]
  struct InvalidError;

  impl From<std::num::ParseIntError> for ParseError {
    fn from(_error: std::num::ParseIntError) -> Self {
      ParseError {}
    }
  }

  impl From<std::char::ParseCharError> for ParseError {
    fn from(_error: std::char::ParseCharError) -> Self {
      ParseError {}
    }
  }

  #[derive(Debug)]
  pub struct Password {
    pub start: u8,
    pub end: u8,
    pub letter: char,
    pub password: String,
  }

  pub trait IsValid {
    fn is_valid(&self) -> bool;
    fn filter_valid(&self) -> Option<&Self>
    where
      Self: Sized;
  }

  impl FromStr for Password {
    type Err = ParseError;

    fn from_str(rule: &str) -> Result<Self, Self::Err> {
      let rule_parts: Vec<&str> = rule.split(" ").collect();
      let range: Vec<&str> = rule_parts[0].split("-").collect();

      let start: u8 = u8::from_str_radix(&range[0], 10)?;
      let end: u8 = u8::from_str_radix(&range[1], 10)?;
      let letter: char = char::from_str(&rule_parts[1].strip_suffix(":").ok_or(ParseError)?)?;
      let password: &str = &rule_parts[2];

      Ok(Password {
        start,
        end,
        letter,
        password: password.to_owned(),
      })
    }
  }
}

mod policy_sled_rental {
  use data::Password;

  fn is_valid(pw: &Password) -> bool {
    let count = pw.password.matches(pw.letter).count();
    count >= pw.start.into() && count <= pw.end.into()
  }

  pub fn validate(pw: &Password) -> Option<&Password> {
    match is_valid(pw) {
      true => Some(pw),
      false => None,
    }
  }
}

mod policy_toboggan_corporate {
  use data::Password;

  fn is_valid(pw: &Password) -> bool {
    let start_match: bool = pw.password.chars().nth((pw.start - 1).into()).unwrap() == pw.letter;
    let end_match: bool = pw.password.chars().nth((pw.end - 1).into()).unwrap() == pw.letter;

    start_match ^ end_match
  }

  pub fn validate(pw: &Password) -> Option<&Password> {
    match is_valid(pw) {
      true => Some(pw),
      false => None,
    }
  }
}

use data::Password;
use std::fs;
use std::str::FromStr;

fn main() {
  let raw: String = fs::read_to_string("input.txt").unwrap();

  let rules: Vec<&str> = raw.split("\n").collect();
  let passwords: Vec<Password> = rules
    .iter()
    .map(|s| Password::from_str(s).unwrap())
    .collect();

  let sled_policy_valid_passwords_count = passwords
    .iter()
    .filter_map(|pw| policy_sled_rental::validate(pw))
    .count();

  let toboggan_corporate_policy_valid_passwords_count = passwords
    .iter()
    .filter_map(|pw| policy_toboggan_corporate::validate(pw))
    .count();

  println!("Sled policy: {:?}", sled_policy_valid_passwords_count);
  println!(
    "Toboggan policy: {:?}",
    toboggan_corporate_policy_valid_passwords_count
  );
}
