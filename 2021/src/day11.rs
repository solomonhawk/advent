const X: i8 = 10;
const Y: i8 = 10;

type OctopusMap = [[u8; X as usize]; Y as usize];
type Point = (i8, i8);

// (x, y)
const OFFSETS: [Point; 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[aoc_generator(day11)]
fn input_generator(input: &str) -> OctopusMap {
    let mut result = [[0; Y as usize]; X as usize];

    for (y, line) in input.split("\n").enumerate() {
        for (x, n) in line.bytes().enumerate() {
            result[y][x] = (n - 48) as u8;
        }
    }

    result
}

#[aoc(day11, part1)]
fn part1(map: &OctopusMap) -> usize {
    let mut map = map.clone();

    (0..100).map(|_| step_map(&mut map)).sum()
}

#[aoc(day11, part2)]
fn part2(map: &OctopusMap) -> usize {
    let mut map = map.clone();
    let mut iteration = 0;

    loop {
        iteration += 1;

        if step_map(&mut map) == (X * Y) as usize {
            return iteration;
        }
    }
}

fn step_map(map: &mut OctopusMap) -> usize {
    let mut flashed = [[0; Y as usize]; X as usize];

    for point in points_iter() {
        step_octopus(&point, map, &mut flashed);
    }

    flashed
        .iter()
        .flat_map(|r| r.iter().filter(|&v| *v == 1))
        .count()
}

fn step_octopus(point: &Point, map: &mut OctopusMap, flashed: &mut OctopusMap) {
    let (x, y) = point;

    // skip this octopus if it already flashed this step
    if flashed[*y as usize][*x as usize] == 1 {
        return;
    }

    // increment
    map[*y as usize][*x as usize] += 1;

    // handle flash
    if map[*y as usize][*x as usize] == 10 {
        // ensure this octopus doesn't flash twice in one step
        flashed[*y as usize][*x as usize] = 1;

        // for each neighboring octopus, step
        for (nx, ny) in valid_neighbors(point) {
            step_octopus(&(nx, ny), map, flashed);
        }

        // since this ocotpus flashed, reset it's brightness to 0
        map[*y as usize][*x as usize] = 0;
    }
}

fn points_iter() -> impl Iterator<Item = Point> {
    (0..Y).flat_map(|y| (0..X).map(move |x| (x, y)))
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

#[allow(unused)]
fn debug(map: &OctopusMap) {
    for line in map.iter() {
        for n in line.iter() {
            print!("{}", n);
        }

        print!("\n");
    }

    println!("----------");
}
