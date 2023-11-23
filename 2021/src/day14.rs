use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ParseError;

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing polymerizations instructions")
    }
}

type Template = Vec<char>;
type InsertionRules = Vec<(Template, char)>;
type Mapping = HashMap<Template, usize>;

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Result<(Template, InsertionRules), Box<dyn Error>> {
    let mut insertion_rules: InsertionRules = Vec::new();
    let (template, insertions) = input.split_once("\n\n").ok_or(ParseError)?;

    for insertion in insertions.split("\n") {
        let (pattern, interstitial) = insertion.split_once(" -> ").ok_or(ParseError)?;

        insertion_rules.push((
            pattern.chars().collect(),
            interstitial.chars().nth(0).ok_or(ParseError)?,
        ));
    }

    Ok((template.chars().collect(), insertion_rules))
}

#[aoc(day14, part1)]
fn part1(input: &(Template, InsertionRules)) -> usize {
    let (template, insertion_rules) = input;
    let mut mapping: Mapping = initialize_mapping(&template, &insertion_rules);

    for _ in 0..10 {
        replace(&mut mapping, &insertion_rules);
    }

    range(count_occurrences(&mapping))
}

#[aoc(day14, part2)]
fn part2(input: &(Template, InsertionRules)) -> usize {
    let (template, insertion_rules) = input;
    let mut mapping: Mapping = initialize_mapping(&template, &insertion_rules);

    for _ in 0..40 {
        replace(&mut mapping, &insertion_rules);
    }

    range(count_occurrences(&mapping))
}

fn initialize_mapping(template: &Template, insertion_rules: &InsertionRules) -> Mapping {
    // create a mapping of all unique pairs of chars to their associated counts
    // { ['N', 'C']: 1, ['N', 'N']: 0, .. }
    let mut mapping: Mapping = insertion_rules
        .iter()
        .flat_map(|(pattern, _)| pattern.clone())
        .unique()
        .flat_map(|c| vec![c, c])
        .permutations(2)
        .unique()
        .map(|t| (t, 0))
        .collect();

    // populate the mapping with pair occurence counts of starting template
    for pair in template.windows(2) {
        if let Some(x) = mapping.get_mut(pair) {
            *x += 1;
        }
    }

    mapping
}

fn replace(mapping: &mut Mapping, insertion_rules: &InsertionRules) {
    let mut operations: Vec<(Template, isize)> = Vec::new();

    for (pair, count) in mapping.iter().filter(|(_, &count)| count > 0) {
        for (pattern, interstitial) in insertion_rules {
            if pair == pattern {
                // decrement the pair (e.g. [N, N])
                operations.push((pair.to_vec(), *count as isize * -1));
                // increment the new starting pair (e.g. [N, C])
                operations.push((vec![pair[0], *interstitial], *count as isize));
                // increment the new ending pair (e.g. [C, N])
                operations.push((vec![*interstitial, pair[1]], *count as isize));
            }
        }
    }

    for (pair, adjustment) in operations.iter() {
        if let Some(x) = mapping.get_mut(pair) {
            *x = (*x as isize + adjustment) as usize;
        }
    }
}

fn count_occurrences(mapping: &Mapping) -> HashMap<char, usize> {
    let mut occurrences: HashMap<char, usize> = HashMap::new();

    for (pair, count) in mapping.iter() {
        *(occurrences.entry(pair[1]).or_insert(0)) += count; // why does this work? 🤷🏻‍♂️
    }

    occurrences
}

fn range(occurrences: HashMap<char, usize>) -> usize {
    let mut counts: Vec<usize> = occurrences.iter().map(|(_c, count)| *count).collect();

    counts.sort();

    counts[counts.len() - 1] - counts[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let input = input_generator("ABCDE\n\nAB -> C\nDE -> F\nBD -> A").unwrap();
        let expected_template = ['A', 'B', 'C', 'D', 'E'];
        let expected_insertion_rules: InsertionRules = vec![
            (vec!['A', 'B'], 'C'),
            (vec!['D', 'E'], 'F'),
            (vec!['B', 'D'], 'A'),
        ];

        let (template, insertion_rules) = input;

        assert_eq!(template, expected_template);
        assert_eq!(insertion_rules, expected_insertion_rules);
    }

    #[test]
    fn part1_test() {
        let input_text = include_str!("../input/2021/day14.txt");
        let input = input_generator(input_text.trim()).unwrap();

        assert_eq!(part1(&input), 2712);
    }

    #[test]
    fn part2_test() {
        let input_text = include_str!("../input/2021/day14.txt");
        let input = input_generator(input_text.trim()).unwrap();

        assert_eq!(part2(&input), 8336623059567);
    }

    #[test]
    fn count_occurrences_test() {
        let mapping: Mapping = vec![
            (vec!['A', 'B'], 1),
            (vec!['B', 'B'], 2),
            (vec!['C', 'A'], 3),
        ]
        .into_iter()
        .collect();

        let occurrences = count_occurrences(&mapping);

        assert_eq!(*occurrences.get(&'B').unwrap(), 3);
        assert_eq!(*occurrences.get(&'A').unwrap(), 3);
        assert_eq!(occurrences.get(&'C'), None);
    }

    #[test]
    fn range_test() {
        let occurrences: HashMap<char, usize> =
            vec![('A', 1), ('B', 5), ('C', 12)].into_iter().collect();

        assert_eq!(range(occurrences), 11);
    }
}
