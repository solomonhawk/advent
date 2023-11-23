use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ParseError;

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing target designation")
    }
}

type Position = (isize, isize); // x, y

#[derive(Debug)]
pub struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

#[derive(Debug)]
pub struct Probe {
    position: Position,
    vx: isize,
    vy: isize,
}

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Result<Target, Box<dyn Error>> {
    let (x_range, y_range) = input
        .trim_start_matches("target area: ")
        .split_once(", ")
        .ok_or(ParseError)?;
    let (x_min, x_max) = x_range
        .trim_start_matches("x=")
        .split_once("..")
        .ok_or(ParseError)?;
    let (y_min, y_max) = y_range
        .trim_start_matches("y=")
        .split_once("..")
        .ok_or(ParseError)?;

    Ok(Target {
        x_min: x_min.parse()?,
        x_max: x_max.parse()?,
        y_min: y_min.parse()?,
        y_max: y_max.parse()?,
    })
}

#[aoc(day17, part1)]
fn part1(target: &Target) -> isize {
    build_trajectories(&target)
        .max()
        .expect("Could not find a maximum!")
}

#[aoc(day17, part2)]
fn part2(target: &Target) -> usize {
    build_trajectories(&target).count()
}

fn build_trajectories(target: &Target) -> impl Iterator<Item = isize> + '_ {
    (0..=target.x_max)
        .flat_map(move |vx| {
            (target.y_min..target.y_min.abs()).map(move |vy| calculate_trajectory(vx, vy, &target))
        })
        .filter_map(|(hit, max_y)| if hit { Some(max_y) } else { None })
}

fn step_probe(probe: &mut Probe) {
    probe.position.0 += probe.vx;
    probe.position.1 += probe.vy;
    probe.vy -= 1;

    if probe.vx > 0 {
        probe.vx += -1 * (probe.vx / probe.vx.abs());
    }
}

fn calculate_trajectory(vx: isize, vy: isize, target: &Target) -> (bool, isize) {
    let mut max_y = 0;
    let mut p = Probe {
        position: (0, 0),
        vx,
        vy,
    };

    while !is_hit(p.position, &target) && !is_miss(p.position, &target) {
        step_probe(&mut p);
        max_y = max_y.max(p.position.1);
    }

    (is_hit(p.position, &target), max_y)
}

fn is_hit(position: Position, target: &Target) -> bool {
    let (x, y) = position;
    x >= target.x_min && x <= target.x_max && y >= target.y_min && y <= target.y_max
}

fn is_miss(position: Position, target: &Target) -> bool {
    let (x, y) = position;
    x > target.x_max || y < target.y_min
}
