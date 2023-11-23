use std::fmt;
use std::{error::Error, str::FromStr};
#[derive(Debug, Clone, Default)]
struct Entry {
    signals: Vec<Signal>,
    outputs: Vec<String>,
}

#[derive(Debug, Clone)]
struct Signal {
    string: String,
    number: usize,
}

#[derive(Debug)]
struct EntryParseError;
impl Error for EntryParseError {}

impl fmt::Display for EntryParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse Entry")
    }
}

impl FromStr for Entry {
    type Err = EntryParseError;

    fn from_str(s: &str) -> Result<Entry, Self::Err> {
        let (signals, outputs) = s.split_once(" | ").unwrap();

        Ok(Entry {
            signals: signals
                .split(" ")
                .map(|s| Signal {
                    string: s.to_string(),
                    number: signal_to_integer(s),
                })
                .collect(),
            outputs: outputs.split(" ").map(|s| s.to_string()).collect(),
            ..Entry::default()
        })
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Entry> {
    input.split("\n").map(|l| l.parse().unwrap()).collect()
}

#[aoc(day8, part1)]
fn part1(lines: &[Entry]) -> usize {
    lines
        .iter()
        .flat_map(|l| {
            l.outputs.iter().filter(|o| {
                let count = o.chars().count();
                count == 2 || count == 3 || count == 4 || count == 7
            })
        })
        .count()
}

#[aoc(day8, part2)]
fn part2(lines: &[Entry]) -> usize {
    lines.iter().map(decode_line).sum()
}

fn decode_line(entry: &Entry) -> usize {
    let mut translation: [usize; 10] = [0; 10];

    translation[1] = find(&entry.signals, by_length(2));
    translation[4] = find(&entry.signals, by_length(4));
    translation[7] = find(&entry.signals, by_length(3));
    translation[8] = find(&entry.signals, by_length(7));

    for signal in entry.signals.iter().filter(by_length(6)) {
        if overlap(signal, translation[4]) == 4 {
            translation[9] = signal.number;
        } else if overlap(signal, translation[1]) == 2 {
            translation[0] = signal.number;
        } else {
            translation[6] = signal.number;
        }
    }

    for signal in entry.signals.iter().filter(by_length(5)) {
        if overlap(signal, translation[7]) == 3 {
            translation[3] = signal.number;
        } else if overlap(signal, translation[6]) == 5 {
            translation[5] = signal.number;
        } else {
            translation[2] = signal.number;
        }
    }

    entry
        .outputs
        .iter()
        .map(|o| match_signal(o, translation))
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap()
}

/**
 * Takes a signal (like "ebdcfa") and returns an integer value by converting the
 * characters into their ASCII values and summing them.
 */
fn signal_to_integer(signal: &str) -> usize {
    signal.chars().map(|c| 1 << (c as usize - 97)).sum()
}

fn overlap(signal: &Signal, translation: usize) -> u32 {
    (signal.number & translation).count_ones()
}

fn find<T>(ns: &Vec<Signal>, func: T) -> usize
where
    T: Fn(&&Signal) -> bool,
{
    for (i, n) in ns.iter().enumerate() {
        if func(&n) {
            return signal_to_integer(&ns[i].string);
        }
    }

    panic!("Could not find!")
}

// higher-order functions, neeeat
fn by_length(len: usize) -> impl Fn(&&Signal) -> bool {
    move |s: &&Signal| s.string.chars().count() == len
}

/**
 * Matches a signal pattern to a decoded digit. (The sequence of signals might
 * differ but the computed integer value will always be equal).
 */
fn match_signal(signal: &String, translation: [usize; 10]) -> usize {
    for (i, n) in translation.iter().enumerate() {
        if signal_to_integer(signal) == *n {
            return i;
        }
    }

    panic!("Could not match signal")
}
