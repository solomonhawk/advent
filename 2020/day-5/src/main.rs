use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ParseError;

#[derive(Default, Debug, PartialEq)]
struct Seat {
    source: String,
    row: u8,
    column: u8,
    seat: u16,
}

impl FromStr for Seat {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // must be 10 characters
        if s.chars().count() != 10 {
            println!("does not have 10 chars");
            return Err(ParseError);
        }

        // first 7 chars must be either B or F
        for c in s[..7].chars() {
            if c == 'B' || c == 'F' {
                continue;
            }

            return Err(ParseError);
        }

        // last 3 chars must be either L or R
        for c in s[7..].chars() {
            if c == 'L' || c == 'R' {
                continue;
            }

            return Err(ParseError);
        }

        Ok(Seat {
            source: s.to_string(),
            ..Default::default()
        })
    }
}

impl Seat {
    fn find(&mut self) -> &Seat {
        let mut min = 0;
        let mut max = 127;

        // determine row
        for c in self.source[..7].chars() {
            let span = (max - min) / 2;

            match c {
                'F' => max = max - span - 1, // lower
                'B' => min = min + span + 1, // upper
                _ => (),
            }
        }

        self.row = min;

        min = 0;
        max = 7;

        // determine column
        for c in self.source[7..].chars() {
            let span = (max - min) / 2;

            match c {
                'L' => max = max - span - 1,
                'R' => min = min + span + 1,
                _ => (),
            }
        }

        self.column = min;
        self.seat = (self.row as u16) * 8 + (self.column as u16);

        self
    }
}

fn main() {
    let raw = fs::read_to_string("src/input.txt").unwrap();

    let seats = raw
        .split("\n")
        .map(|s| Seat::from_str(&s).ok().unwrap())
        .map(|mut s| s.find().seat);

    let mut open_seat = 0;
    let min_seat = seats.clone().min().unwrap();
    let max_seat = seats.clone().max().unwrap();

    for s in min_seat..max_seat {
        let mut found = false;

        // this seems inefficient
        for c in seats.clone() {
            if s == c {
                found = true;
                break;
            }
        }

        if !found {
            open_seat = s;
        }
    }

    println!(
        "{:?} - {:?}, my seat is: {:?}",
        min_seat, max_seat, open_seat
    );
}

#[test]
fn test_seat_parsing() {
    assert_eq!(
        Ok(Seat {
            source: "BFFFBBFRRR".to_owned(),
            ..Default::default()
        }),
        Seat::from_str("BFFFBBFRRR")
    );

    assert_eq!(Err(ParseError), Seat::from_str("BFFFBBZRRR"));
}

#[test]
fn test_find() {
    assert_eq!(357, Seat::from_str("FBFBBFFRLR").unwrap().find().seat);

    assert_eq!(567, Seat::from_str("BFFFBBFRRR").unwrap().find().seat);

    assert_eq!(119, Seat::from_str("FFFBBBFRRR").unwrap().find().seat);

    assert_eq!(820, Seat::from_str("BBFFBBFRLL").unwrap().find().seat);
}
