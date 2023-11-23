use std::{collections::HashMap, error::Error, fmt, str::FromStr};

#[derive(Debug)]
struct Line(
    u32, /* x1 */
    u32, /* y1 */
    u32, /* x2 */
    u32, /* y2 */
);

#[derive(Debug)]
struct LineParseError;

impl Error for LineParseError {}

impl fmt::Display for LineParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Error parsing line")
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Line, Self::Err> {
        let (start, end) = string.split_once(" -> ").ok_or(LineParseError)?;
        let (x1, y1) = start.split_once(",").ok_or(LineParseError)?;
        let (x2, y2) = end.split_once(",").ok_or(LineParseError)?;

        Ok(Line(x1.parse()?, y1.parse()?, x2.parse()?, y2.parse()?))
    }
}

fn point_range_vh(a: u32, b: u32) -> impl DoubleEndedIterator<Item = u32> {
    if a > b {
        b..a + 1
    } else {
        a..b + 1
    }
}

// ??????? bunch of dumb shit dealing with normalizing point iterators as ranges
fn point_range_diag(line: &Line) -> Box<dyn Iterator<Item = (u32, u32)>> {
    let Line(x1, y1, x2, y2) = *line;

    let h = point_range_vh(x1, x2);
    let v = point_range_vh(y1, y2);

    // xs are reversed, so we need to reverse ys
    if x1 > x2 && y1 < y2 {
        return Box::new(h.zip(v.rev()));
    }

    // ys are reversed, so we need to reverse xs
    if y1 > y2 && x1 < x2 {
        return Box::new(h.rev().zip(v));
    }

    Box::new(h.zip(v))
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<Line> {
    input.split("\n").map(|l| l.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
fn part1(lines: &[Line]) -> usize {
    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();

    for line in lines {
        let Line(x1, y1, x2, y2) = &line;

        if x1 == x2 {
            // vertical
            for y in point_range_vh(*y1, *y2) {
                *(counts.entry((*x1, y)).or_insert(0)) += 1;
            }
        } else if y1 == y2 {
            // horizontal
            for x in point_range_vh(*x1, *x2) {
                *(counts.entry((x, *y1)).or_insert(0)) += 1;
            }
        }
    }

    counts.into_values().filter(|&v| v > 1).count()
}

#[aoc(day5, part2)]
fn part2(lines: &[Line]) -> usize {
    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();

    for line in lines {
        let Line(x1, y1, x2, y2) = &line;

        if x1 == x2 {
            // vertical
            for y in point_range_vh(*y1, *y2) {
                *(counts.entry((*x1, y)).or_insert(0)) += 1;
            }
        } else if y1 == y2 {
            // horizontal
            for x in point_range_vh(*x1, *x2) {
                *(counts.entry((x, *y1)).or_insert(0)) += 1;
            }
        } else {
            // diagonal
            for (x, y) in point_range_diag(line) {
                *(counts.entry((x, y)).or_insert(0)) += 1;
            }
        }
    }

    counts.into_values().filter(|&v| v > 1).count()
}
