use std::collections::HashSet;

const X: i8 = 100;
const Y: i8 = 100;

type HeightMap = [[i8; X as usize]; Y as usize];
type Point = (i8, i8);

// x, y
const OFFSETS: [Point; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> HeightMap {
    let mut result = [[0; X as usize]; Y as usize];

    for (y, line) in input.split("\n").enumerate() {
        for (x, n) in line.bytes().enumerate() {
            result[y][x] = (n - 48) as i8;
        }
    }

    result
}

#[aoc(day9, part1)]
pub fn part1(map: &HeightMap) -> isize {
    points_iter()
        .filter(|point| is_local_minimum(point, &map))
        .map(|point| risk_level(&point, &map))
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(map: &HeightMap) -> usize {
    let mut basin_sizes: Vec<usize> = points_iter()
        .filter(|point| is_local_minimum(point, &map))
        .map(|point| find_basin(&point, &map))
        .map(|basin| basin.len())
        .collect::<Vec<usize>>();

    basin_sizes.sort();
    basin_sizes.reverse();

    basin_sizes[..3].iter().product()
}

fn find_basin(origin: &Point, map: &HeightMap) -> Vec<Point> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut candidates = valid_neighbors(origin);

    visited.insert(*origin);

    while candidates.len() > 0 {
        // grab first candidate as center point
        let point = candidates.swap_remove(0);

        // mark this candidate as visited
        visited.insert(point);

        // iterate over the neighbors, filtering out those that are height 9
        for (x, y) in valid_neighbors(&point) {
            if visited.contains(&(x, y)) || map[y as usize][x as usize] == 9 {
                continue;
            }

            // add each neighbor as a new candidate to traverse, make
            // sure we don't add the same candidate more than once
            visited.insert((x, y));
            candidates.push((x, y));
        }
    }

    visited.into_iter().collect()
}

fn points_iter() -> impl Iterator<Item = Point> {
    (0..Y).flat_map(|y| (0..X).map(move |x| (x, y)))
}

fn is_local_minimum(point: &Point, map: &HeightMap) -> bool {
    let (x, y) = point;
    let height = map[*y as usize][*x as usize];

    for (x, y) in valid_neighbors(point) {
        if map[y as usize][x as usize] <= height {
            return false;
        }
    }

    true
}

fn risk_level(point: &Point, map: &HeightMap) -> isize {
    (map[point.1 as usize][point.0 as usize] + 1) as isize
}

fn valid_neighbors(origin: &Point) -> Vec<Point> {
    let (x, y) = origin;

    OFFSETS
        .iter()
        .map(|(a, b)| (x + a, y + b))
        .filter(|(x, y)| valid_point(*x, *y))
        .collect()
}

fn valid_point(x: i8, y: i8) -> bool {
    x >= 0 && x < X && y >= 0 && y < Y
}
