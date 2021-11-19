use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::process;

fn part1() {
    let mut total = 0;
    let input = fs::read_to_string("input.txt").unwrap_or_else(|error| {
        println!("Invalid input! {}", error);
        process::exit(1)
    });

    let groups = input.split("\n\n").map(|p| str::replace(p, "\n", ""));

    for s in groups {
        let mut seen = HashSet::new();

        for c in s.chars().into_iter() {
            seen.insert(c);
        }

        total = total + seen.len();
    }

    println!("{:?}", total)
}

fn intersections<T: Clone + Eq + Hash>(hash_sets: Vec<HashSet<T>>) -> HashSet<T> {
    if hash_sets.iter().count() < 1 {
        // print!("no elements");
        return HashSet::new();
    }

    if hash_sets.iter().count() < 2 {
        // print!("only 1 element");
        return hash_sets[0].clone();
    }

    let mut common_set = hash_sets[0].clone();

    for hash_set in hash_sets {
        common_set = common_set.intersection(&hash_set).cloned().collect();
    }

    common_set
}

fn part2() {
    let mut total = 0;
    let input = fs::read_to_string("input.txt").unwrap_or_else(|error| {
        println!("Invalid input! {}", error);
        process::exit(1)
    });

    let groups = input
        .split("\n\n")
        .map(|p| p.split("\n").collect::<Vec<&str>>());

    for group in groups {
        let hash_sets = group
            .iter()
            .map(|g| {
                let mut seen = HashSet::new();

                for c in g.chars().into_iter() {
                    seen.insert(c);
                }

                seen
            })
            .collect();

        total = total + intersections(hash_sets).len();
    }

    println!("{:?}", total);
}

fn main() {
    part2()
}
