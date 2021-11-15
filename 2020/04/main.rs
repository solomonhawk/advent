use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct ParseError;

impl From<std::num::ParseIntError> for ParseError {
  fn from(_error: std::num::ParseIntError) -> Self {
    // should probably actually wrap the error...
    ParseError {}
  }
}

#[derive(Debug)]
struct HexColor {
  r: u8,
  g: u8,
  b: u8
}

impl FromStr for HexColor {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.strip_prefix("#") {
      Some(hex) => Ok(HexColor{
        r: u8::from_str_radix(&hex[..2], 16)?,
        g: u8::from_str_radix(&hex[2..4], 16)?,
        b: u8::from_str_radix(&hex[4..], 16)?}
      ),
      None => Err(ParseError)
    }
  }
}

#[derive(Debug)]
enum Height {
  Cm(u8),
  In(u8),
}

impl FromStr for Height {
  type Err = ParseError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    if input.ends_with("cm") {
      let value = input.strip_suffix("cm").ok_or(ParseError).unwrap();
      return Ok(Height::Cm(u8::from_str_radix(value, 10)?))
    }

    if input.ends_with("in") {
      let value = input.strip_suffix("in").ok_or(ParseError).unwrap();
      return Ok(Height::In(u8::from_str_radix(value, 10)?))
    }

    Err(ParseError{})
  }
}

#[derive(Debug)]
enum EyeColor {
  Amb,
  Blu,
  Brn,
  Gry,
  Grn,
  Hzl,
  Oth
}

impl FromStr for EyeColor {
  type Err = ParseError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    match input {
      "amb" => Ok(EyeColor::Amb),
      "blu" => Ok(EyeColor::Blu),
      "brn" => Ok(EyeColor::Brn),
      "gry" => Ok(EyeColor::Gry),
      "grn" => Ok(EyeColor::Grn),
      "hzl" => Ok(EyeColor::Hzl),
      "oth" => Ok(EyeColor::Oth),
      _ => Err(ParseError{})
    }
  }
}

fn parse_pid(s: &str) -> Option<String> {
  // must have leading 0's padding to 9 digits
  if s.len() != 9 {
    return None
  }

  match u32::from_str_radix(s, 10) {
    Ok(_) => Some(s.to_string()),
    Err(_) => None
  }
}

#[derive(Debug, Default)]
struct Passport {
  byr: Option<u16>,      // (Birth Year)
  iyr: Option<u16>,      // (Issue Year)
  eyr: Option<u16>,      // (Expiration Year)
  hgt: Option<Height>,   // (Height)
  hcl: Option<HexColor>, // (Hair Color)
  ecl: Option<EyeColor>, // (Eye Color)
  pid: Option<String>,   // (Passport ID)
  cid: Option<String>,   // (Country ID)
}

impl FromStr for Passport {
  type Err = ParseError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let mut passport = Passport::default();

    for field in input.split(" ") {
      let mut values = field.split(":");

      match (values.next(), values.next()) {
        (Some("byr"), Some(v)) => passport.byr = Some(u16::from_str_radix(v, 10)?),
        (Some("iyr"), Some(v)) => passport.iyr = Some(u16::from_str_radix(v, 10)?),
        (Some("eyr"), Some(v)) => passport.eyr = Some(u16::from_str_radix(v, 10)?),
        (Some("hgt"), Some(v)) => passport.hgt = Some(Height::from_str(v)?),
        (Some("hcl"), Some(v)) => passport.hcl = Some(HexColor::from_str(v)?),
        (Some("ecl"), Some(v)) => passport.ecl = Some(EyeColor::from_str(v)?),
        (Some("pid"), Some(v)) => passport.pid = parse_pid(v),
        (Some("cid"), Some(v)) => passport.cid = Some(v.to_string()),
        (_, _) => (),
      };
    }

    Ok(passport)
  }
}

impl Passport {
  fn is_valid(&self) -> bool {
    match self {
      Passport {
        byr: Some(byr),
        iyr: Some(iyr),
        eyr: Some(eyr),
        hgt: Some(hgt),
        hcl: Some(_),
        ecl: Some(_),
        pid: Some(_),
        ..
      } => {
        if byr < &1920 || byr > &2002 {
          return false
        }

        if iyr < &2010 || iyr > &2020 {
          return false
        }

        if eyr < &2020 || eyr > &2030 {
          return false
        }

        match hgt {
          Height::Cm(h) => {
            if h < &150 || h > &193 {
              return false
            }
          },
          Height::In(h) => {
            if h < &59 || h > &76 {
              return false
            }
          }
        }

        true
      },
      _ => false,
    }
  }
}

fn main() {
  let raw: String = fs::read_to_string("input.txt").unwrap();

  let count = raw
    .split("\n\n")
    .map(|p| str::replace(p, "\n", " "))
    .map(|p| Passport::from_str(&p))
    .filter(|r| match r {
      Ok(p) => Passport::is_valid(&p),
      Err(_) => false
    })
    .count();

  println!("{:?}", count);
}
