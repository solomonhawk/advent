#![allow(dead_code, unused_variables)]

// --- Day 17: Conway Cubes ---
// As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy source aboard one of their super-secret imaging satellites.

// The experimental energy source is based on cutting-edge technology: a set of Conway Cubes contained in a pocket dimension! When you hear it's having problems, you can't help but agree to take a look.

// The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional coordinate (x,y,z), there exists a single cube which is either active or inactive.

// In the initial state of the pocket dimension, almost all cubes start inactive. The only exception to this is a small flat region of cubes (your puzzle input); the cubes in this region start in the specified active (#) or inactive (.) state.

// The energy source then proceeds to boot up by executing six cycles.

// Each cube only ever considers its neighbors: any of the 26 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.

// During a cycle, all cubes simultaneously change their state according to the following rules:

// If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
// If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
// The engineers responsible for this experimental energy source would like you to simulate the pocket dimension and determine what the configuration of cubes should be at the end of the six-cycle boot process.

// For example, consider the following initial state:

// .#.
// ..#
// ###
// Even though the pocket dimension is 3-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the 3-dimensional space.)

// Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z coordinate (and the frame of view follows the active cells in each cycle):

// Before any cycles:

// z=0
// .#.
// ..#
// ###

// After 1 cycle:

// z=-1
// #..
// ..#
// .#.

// z=0
// #.#
// .##
// .#.

// z=1
// #..
// ..#
// .#.

// After 2 cycles:

// z=-2
// .....
// .....
// ..#..
// .....
// .....

// z=-1
// ..#..
// .#..#
// ....#
// .#...
// .....

// z=0
// ##...
// ##...
// #....
// ....#
// .###.

// z=1
// ..#..
// .#..#
// ....#
// .#...
// .....

// z=2
// .....
// .....
// ..#..
// .....
// .....

// After 3 cycles:

// z=-2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......

// z=-1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...

// z=0
// ...#...
// .......
// #......
// .......
// .....##
// .##.#..
// ...#...

// z=1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...

// z=2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......
// After the full six-cycle boot process completes, 112 cubes are left in the active state.

// Starting with your given initial configuration, simulate six cycles. How many cubes are left in the active state after the sixth cycle?

// --- Part Two ---
// For some reason, your simulated results don't match what the experimental energy source engineers expected. Apparently, the pocket dimension actually has four spatial dimensions, not three.

// The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional coordinate (x,y,z,w), there exists a single cube (really, a hypercube) which is still either active or inactive.

// Each cube only ever considers its neighbors: any of the 80 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube at x=0,y=2,z=3,w=4, and so on.

// The initial state of the pocket dimension still consists of a small flat region of cubes. Furthermore, the same rules for cycle updating still apply: during each cycle, consider the number of active neighbors of each cube.

// For example, consider the same initial state as in the example above. Even though the pocket dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)

// Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z and w coordinate:

// Before any cycles:

// z=0, w=0
// .#.
// ..#
// ###

// After 1 cycle:

// z=-1, w=-1
// #..
// ..#
// .#.

// z=0, w=-1
// #..
// ..#
// .#.

// z=1, w=-1
// #..
// ..#
// .#.

// z=-1, w=0
// #..
// ..#
// .#.

// z=0, w=0
// #.#
// .##
// .#.

// z=1, w=0
// #..
// ..#
// .#.

// z=-1, w=1
// #..
// ..#
// .#.

// z=0, w=1
// #..
// ..#
// .#.

// z=1, w=1
// #..
// ..#
// .#.

// After 2 cycles:

// z=-2, w=-2
// .....
// .....
// ..#..
// .....
// .....

// z=-1, w=-2
// .....
// .....
// .....
// .....
// .....

// z=0, w=-2
// ###..
// ##.##
// #...#
// .#..#
// .###.

// z=1, w=-2
// .....
// .....
// .....
// .....
// .....

// z=2, w=-2
// .....
// .....
// ..#..
// .....
// .....

// z=-2, w=-1
// .....
// .....
// .....
// .....
// .....

// z=-1, w=-1
// .....
// .....
// .....
// .....
// .....

// z=0, w=-1
// .....
// .....
// .....
// .....
// .....

// z=1, w=-1
// .....
// .....
// .....
// .....
// .....

// z=2, w=-1
// .....
// .....
// .....
// .....
// .....

// z=-2, w=0
// ###..
// ##.##
// #...#
// .#..#
// .###.

// z=-1, w=0
// .....
// .....
// .....
// .....
// .....

// z=0, w=0
// .....
// .....
// .....
// .....
// .....

// z=1, w=0
// .....
// .....
// .....
// .....
// .....

// z=2, w=0
// ###..
// ##.##
// #...#
// .#..#
// .###.

// z=-2, w=1
// .....
// .....
// .....
// .....
// .....

// z=-1, w=1
// .....
// .....
// .....
// .....
// .....

// z=0, w=1
// .....
// .....
// .....
// .....
// .....

// z=1, w=1
// .....
// .....
// .....
// .....
// .....

// z=2, w=1
// .....
// .....
// .....
// .....
// .....

// z=-2, w=2
// .....
// .....
// ..#..
// .....
// .....

// z=-1, w=2
// .....
// .....
// .....
// .....
// .....

// z=0, w=2
// ###..
// ##.##
// #...#
// .#..#
// .###.

// z=1, w=2
// .....
// .....
// .....
// .....
// .....

// z=2, w=2
// .....
// .....
// ..#..
// .....
// .....
// After the full six-cycle boot process completes, 848 cubes are left in the active state.

// Starting with your given initial configuration, simulate six cycles in a 4-dimensional space. How many cubes are left in the active state after the sixth cycle?

use std::{collections::HashMap, fs};

type X = i32;
type Y = i32;
type Z = i32;
type W = i32;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Cube(X, Y, Z, W);

#[derive(Debug)]
struct ConwayDimension {
    cubes: HashMap<Cube, bool>,
}

impl ConwayDimension {
    fn new(input: &str) -> Self {
        let mut width: i32 = 0;

        let total_chars: i32 = input.chars().count() as i32;
        let cube_rows = input.split("\n");

        let cubes: HashMap<Cube, bool> = cube_rows
            .enumerate()
            .flat_map(|(y, row)| {
                if width == 0 {
                    width = row.len() as i32;
                }

                row.chars().enumerate().filter_map(move |(x, s)| match s {
                    '#' => Some((Cube(x as X, y as Y, 0, 0), true)),
                    '.' => None,
                    _ => None,
                })
            })
            .collect();

        ConwayDimension { cubes }
    }

    fn cycle(&mut self) {
        let mut next_cubes = self.cubes.clone();

        // for each active cube in simulation
        for (cube, _) in &self.cubes {
            // get all the neighboring positions excluding
            let neighbor_positions = self.neighbor_positions(&cube);
            let live_neighbors = self.live_neighbors(&neighbor_positions);

            // die of under/overcrowding
            if !(live_neighbors == 2 || live_neighbors == 3) {
                next_cubes.remove(&cube);
            }

            // check edge positions
            for neighbor in &neighbor_positions {
                // if there's a live cube here already, skip it
                if self.has_alive_cube(&neighbor) {
                    continue;
                }

                // if exactly 3 neighbors, spawn a cube here
                if self.live_neighbors(&self.neighbor_positions(&neighbor)) == 3 {
                    next_cubes.insert(*neighbor, true);
                }
            }
        }

        self.cubes = next_cubes;
    }

    fn has_alive_cube(&self, cube: &Cube) -> bool {
        self.cubes.contains_key(cube)
    }

    fn neighbor_positions(&self, cube: &Cube) -> Vec<Cube> {
        let mut positions = Vec::new();
        let Cube(ox, oy, oz, ow) = cube;

        for z in (oz - 1)..(oz + 2) {
            for x in (ox - 1)..(ox + 2) {
                for y in (oy - 1)..(oy + 2) {
                    for w in (ow - 1)..(ow + 2) {
                        if (x, y, z, w) == (*ox, *oy, *oz, *ow) {
                            continue;
                        }

                        positions.push(Cube(x, y, z, w))
                    }
                }
            }
        }

        positions
    }

    fn live_neighbors(&self, positions: &Vec<Cube>) -> u32 {
        let mut count = 0;

        for cube in positions {
            if self.has_alive_cube(&cube) {
                count += 1;
            }
        }

        count
    }
}

fn main() {
    let mut c = ConwayDimension::new(
        &fs::read_to_string("src/input.txt")
            .unwrap()
            .trim_end_matches("\n"),
    );

    for _ in 0..6 {
        c.cycle();
    }

    println!("{:#?}", c.cubes.keys().count());
}
