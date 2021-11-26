#![allow(dead_code)]
// --- Day 7: Handy Haversacks ---
// You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.

// Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!

// For example, consider the following rules:

// light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.
// These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

// You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

// In the above rules, the following options would be available to you:

// A bright white bag, which can hold your shiny gold bag directly.
// A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
// A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
// A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
// So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

// How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::process;

#[derive(Debug, Clone)]
struct ParseError;

#[derive(Debug, Clone)]
struct BagRule<'a> {
    color: &'a str,
    contents: HashMap<&'a str, BagContents<'a>>,
}

#[derive(Debug, Clone)]
struct BagContents<'a> {
    color: &'a str,
    count: u32,
}

#[derive(Debug)]
struct BagManager<'a> {
    rules: HashMap<&'a str, BagRule<'a>>,
}

impl<'a> BagManager<'a> {
    fn parse_rules(string: &'a str) -> BagManager<'a> {
        let rules = string
            .split("\n")
            .map(|line| BagManager::parse_line(line))
            .filter_map(|r| r.ok());

        BagManager {
            rules: rules.map(|r| (r.color, r)).collect(),
        }
    }

    fn parse_line(line: &str) -> Result<BagRule, ParseError> {
        lazy_static! {
            static ref LINE_PATTERN: Regex =
                Regex::new(r"^(?P<count>\d) (?P<color>\w+ \w+)$").unwrap();
        }

        let parts: Vec<&str> = line.split(" bags contain ").collect();
        let color = parts[0];
        let rules = parts[1]
            .split(", ")
            .map(|s| {
                s.trim_end_matches(".")
                    .trim_end_matches(" bags")
                    .trim_end_matches(" bag")
            })
            .filter_map(|rule| {
                if let Some(c) = LINE_PATTERN.captures(rule) {
                    Some(BagContents {
                        count: u32::from_str_radix(c.name("count").map(|x| x.as_str())?, 10)
                            .unwrap(),
                        color: c.name("color").map(|x| x.as_str())?,
                    })
                } else {
                    None
                }
            });

        Ok(BagRule {
            color: color,
            contents: rules.map(|r| (r.color, r)).collect(),
        })
    }

    // this is very inefficient
    fn containing(&'a self, color: &str) -> Option<Vec<&'a str>> {
        let s: Vec<&str> = self
            .rules
            .iter()
            .filter_map(|(&c, rule)| {
                if rule.contents.contains_key(color) {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();

        if s.len() > 0 {
            Some(s)
        } else {
            None
        }
    }

    fn find_bags_containing(&self, color: &str) -> HashSet<&'_ str> {
        match self.containing(color) {
            Some(containers) => containers
                .to_vec()
                .into_iter()
                .chain(containers.iter().flat_map(|c| self.find_bags_containing(c)))
                .collect(),
            None => HashSet::new(),
        }
    }

    fn find_bags_contained_by(&self, color: &str) -> u32 {
        self.rules[color]
            .contents
            .iter()
            .map(|(&c, contents)| contents.count + contents.count * self.find_bags_contained_by(c))
            .sum()
    }
}

fn part1(manager: &BagManager) {
    let containers: HashSet<&str> = manager.find_bags_containing("shiny gold");

    println!(
        "Bags that can contain at least 1 shiny gold bag: {:#?}",
        containers.len()
    );
}

fn part2(manager: &BagManager) {
    let result = manager.find_bags_contained_by("shiny gold");

    println!("{:#?}", result);
}

fn main() {
    let rules = fs::read_to_string("input.txt").unwrap_or_else(|error| {
        println!("Failed to parse input. {}", error);
        process::exit(1);
    });

    let manager = BagManager::parse_rules(&rules);

    // part1(&manager)
    part2(&manager)
}
