use core::fmt;
use std::cmp;
use std::error::Error;
use std::str::FromStr;

/**
 * Fun fact, this problem does fit in the 2mb of stack-allocated memory, but could
 * easily exceed it if the maximum points were slightly further out.
 *
 * 1024 * 1024 * 2 = 2,097,152 bytes (stack limit).
 *
 * The array being stored here is X * Y bytes large (bool's are u8 or
 * 1 byte). 1400 * 900 = 1,260,000 bytes.
 *
 * The maximum square array size that will fit into stack memory is ~1418 ^ 2 (the
 * square root of 2mb in bytes).
 */
const Y: usize = 1400;
const X: usize = 900;

#[derive(Debug)]
pub struct Paper {
    width: usize,         // width of occupied region of the "arena"
    height: usize,        // height of occupied region of the "arena"
    dots: [[bool; Y]; X], // statically sized array that *should* hold enough values
    folds: Vec<Fold>,
}

#[derive(Clone, Debug)]
pub enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl FromStr for Fold {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Fold, Self::Err> {
        let rule = s.trim_start_matches("fold along ");
        let (axis, amount) = rule.split_once("=").ok_or(PaperParseError)?;

        match axis {
            "y" => Ok(Fold::Vertical(amount.parse()?)),
            "x" => Ok(Fold::Horizontal(amount.parse()?)),
            _ => Err(Box::new(PaperParseError)),
        }
    }
}

#[derive(Debug)]
pub struct PaperParseError;

impl Error for PaperParseError {}

impl fmt::Display for PaperParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse paper")
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Result<Paper, Box<dyn Error>> {
    let (points, folds) = input.split_once("\n\n").ok_or(PaperParseError)?;

    let folds: Result<Vec<Fold>, Box<dyn Error>> = folds.split("\n").map(|f| f.parse()).collect();

    let mut paper = Paper {
        width: 0,
        height: 0,
        dots: [[false; Y]; X],
        folds: folds?,
    };

    let mut max_x = 0;
    let mut max_y = 0;

    for point in points.split("\n") {
        let (x, y) = point.split_once(",").ok_or(PaperParseError)?;

        let x: usize = x.parse()?;
        let y: usize = y.parse()?;

        max_x = cmp::max(max_x, x);
        max_y = cmp::max(max_y, y);

        paper.dots[y][x] = true;
    }

    paper.width = max_x;
    paper.height = max_y;

    Ok(paper)
}

#[aoc(day13, part1)]
pub fn part1(paper: &Paper) -> usize {
    // annoying, can't take a mutable reference as an argument here b/c
    // cargo-aoc crate doesn't work that way so gotta make our own mutable copy
    let mut paper = Paper {
        width: paper.width,
        height: paper.height,
        dots: paper.dots.clone(),
        folds: paper.folds.clone(),
    };

    apply_fold(&mut paper);

    count_dots(&paper)
}

#[aoc(day13, part2)]
pub fn part2(paper: &Paper) -> bool {
    // annoying, can't take a mutable reference as an argument here b/c
    // cargo-aoc crate doesn't work that way so gotta make our own mutable copy
    let mut paper = Paper {
        width: paper.width,
        height: paper.height,
        dots: paper.dots.clone(),
        folds: paper.folds.clone(),
    };

    while paper.folds.len() > 0 {
        apply_fold(&mut paper);
    }

    debug(&paper);

    true
}

fn apply_fold(paper: &mut Paper) {
    let fold = &paper.folds[0];

    let (width, height) = match fold {
        Fold::Horizontal(amount) => (amount - 1, paper.height),
        Fold::Vertical(amount) => (paper.width, amount - 1),
    };

    match fold {
        Fold::Horizontal(amount) => {
            for y in 0..=height {
                for x in amount + 1..=paper.width {
                    paper.dots[y][x - (x - amount) * 2] |= paper.dots[y][x];
                }
            }
        }
        Fold::Vertical(amount) => {
            for y in amount + 1..=paper.height {
                for x in 0..=width {
                    paper.dots[y - (y - amount) * 2][x] |= paper.dots[y][x];
                }
            }
        }
    }

    paper.width = width;
    paper.height = height;
    paper.folds = paper.folds[1..].to_vec();
}

fn count_dots(paper: &Paper) -> usize {
    let mut count = 0;

    for y in 0..=paper.height {
        for x in 0..=paper.width {
            if paper.dots[y][x] {
                count += 1;
            }
        }
    }

    count
}

fn debug(paper: &Paper) {
    for y in 0..=paper.height {
        for x in 0..=paper.width {
            if paper.dots[y][x] {
                print!("#");
            } else {
                print!(".");
            }
        }

        print!("\n");
    }
}
