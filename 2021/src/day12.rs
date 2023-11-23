#![allow(unused)]
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

pub type CaveId = String;

pub struct CaveSystem {
    caves: HashMap<CaveId, Cave>,
}

#[derive(Debug)]
pub struct CaveParseError;

impl Error for CaveParseError {}

impl fmt::Display for CaveParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse CaveSystem")
    }
}

#[derive(Debug, PartialEq)]
pub enum CaveSize {
    Small,
    Large,
}

#[derive(Debug)]
pub struct Cave {
    id: CaveId,
    size: CaveSize,
    edges: Vec<CaveId>,
}

impl FromStr for CaveSystem {
    type Err = CaveParseError;

    fn from_str(s: &str) -> Result<CaveSystem, Self::Err> {
        let mut caves: HashMap<String, Cave> = HashMap::new();

        for passage in s.split("\n") {
            let (source, dest) = passage.split_once("-").ok_or(CaveParseError)?;

            insert_or_update(&mut caves, source, dest);
            insert_or_update(&mut caves, dest, source);
        }

        Ok(CaveSystem { caves })
    }
}

fn insert_or_update(caves: &mut HashMap<String, Cave>, source: &str, dest: &str) {
    if !caves.contains_key(source) {
        caves.insert(
            source.to_string(),
            Cave {
                id: source.to_string(),
                size: cave_size(&source),
                edges: vec![dest.to_string()],
            },
        );
    } else {
        if let Some(cave) = caves.get_mut(source) {
            cave.edges.push(dest.to_string());
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Result<CaveSystem, Box<dyn Error>> {
    Ok(input.parse()?)
}

#[aoc(day12, part1)]
pub fn part1(cave_system: &CaveSystem) -> usize {
    let mut all_paths: Vec<Vec<String>> = Vec::new();

    build_paths(
        &cave_system.caves,
        String::from("start"),
        &mut all_paths,
        &mut vec![],
        1,
        true,
    );

    all_paths.len()
}

#[aoc(day12, part2)]
pub fn part2(cave_system: &CaveSystem) -> usize {
    let mut all_paths: Vec<Vec<String>> = Vec::new();

    build_paths(
        &cave_system.caves,
        String::from("start"),
        &mut all_paths,
        &mut vec![],
        2,
        false,
    );

    all_paths.len()
}

fn build_paths(
    caves: &HashMap<CaveId, Cave>,
    cave_id: CaveId,
    all_paths: &mut Vec<Vec<String>>,
    path: &mut Vec<String>,
    max_small_cave_visits: usize,
    found_any_duplicate: bool,
) {
    let cave = caves.get(&cave_id).unwrap();

    // add this cave to the path
    path.push(cave_id.clone());

    // if it was the "end" cave, push the `path` to the result list and return
    if is_end(&cave) {
        all_paths.push(path.to_vec());
        return;
    }

    // if we've added a duplicate small cave to the one we just added
    let has_duplicate = is_duplicate_small_cave(&cave, path, max_small_cave_visits);

    // recalculate max_visits, but only ever subtract 1 once
    let max_visits = if !found_any_duplicate && has_duplicate {
        max_small_cave_visits - 1
    } else {
        max_small_cave_visits
    };

    // for each connected cave
    for edge_id in cave.edges.iter() {
        let edge_cave = caves.get(edge_id).unwrap();

        // if it's the "start" cave or a duplicate small cave (respecting max visits), skip it
        if is_start(&edge_cave) || is_duplicate_small_cave(&edge_cave, path, max_visits) {
            continue;
        }

        // continue building the path along this connection carrying forward the
        // information about whether we've encountered a duplicate already
        build_paths(
            caves,
            edge_id.to_string(),
            all_paths,
            &mut path.to_vec(),
            max_visits,
            found_any_duplicate || has_duplicate,
        );
    }
}

fn is_start(cave: &Cave) -> bool {
    cave.id == "start"
}

fn is_end(cave: &Cave) -> bool {
    cave.id == "end"
}

fn is_duplicate_small_cave(cave: &Cave, path: &Vec<String>, max_visits: usize) -> bool {
    cave.size == CaveSize::Small && (path.iter().filter(|id| **id == cave.id).count() >= max_visits)
}

fn cave_size(s: &str) -> CaveSize {
    if is_uppercase(s) {
        CaveSize::Large
    } else {
        CaveSize::Small
    }
}

fn is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_uppercase())
}
