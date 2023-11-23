use std::error::Error;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split("\n").map(|s| s.to_string()).collect()
}

#[derive(Debug)]
struct DiagnosticReport {
    entries: Vec<String>,
}

impl DiagnosticReport {
    fn iter<'a>(&'a self) -> impl Iterator<Item = Vec<usize>> + 'a {
        DiagnosticReportIter {
            cursor: 0,
            report: self,
        }
    }
}

#[derive(Debug)]
struct DiagnosticReportIter<'a> {
    cursor: usize,
    report: &'a DiagnosticReport,
}

impl<'a> Iterator for DiagnosticReportIter<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let entries_len = self.report.entries.len();

        if entries_len == 0 || self.cursor == self.report.entries[0].len() {
            return None;
        }

        let mut result = Vec::with_capacity(entries_len);

        for s in self.report.entries.iter() {
            result.push(
                s.chars()
                    .nth(self.cursor)
                    .expect("Failed to unwrap element")
                    .to_digit(10)
                    .unwrap() as usize,
            );
        }

        self.cursor += 1;

        Some(result)
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> Result<u32, Box<dyn Error>> {
    let report = DiagnosticReport {
        entries: input.to_vec(),
    };

    let mut gamma = 0;
    let mut epsilon = 0;

    let width = input[0].len();

    for (i, n) in report.iter().enumerate() {
        let result: isize = n
            .iter()
            .map(|v| match v {
                0 => -1,
                1 => 1,
                _ => panic!("Unexpected value"),
            })
            .sum();

        if result > 0 {
            gamma += 1 << (width - i - 1);
        } else {
            epsilon += 1 << (width - i - 1);
        }
    }

    Ok(gamma * epsilon)
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> usize {
    let co2_rating = calculate_rating(&mut input.to_vec(), |c| c >= 0);
    let o2_rating = calculate_rating(&mut input.to_vec(), |c| c < 0);

    co2_rating * o2_rating
}

fn calculate_rating<F: Fn(isize) -> bool>(entries: &mut Vec<String>, retain: F) -> usize {
    let mut position = 0;

    while entries.len() > 1 {
        let report = DiagnosticReport {
            entries: entries.to_vec(),
        };

        // sum 1's and 0's, mapping 0's to -1 - a positive result means there
        // were more 1's than 0's
        let column_sum: isize = calculate_column_sum(report, position);

        // filter entries so that only entries remain where the bit at
        // `position` matches the most common bit
        entries.retain(|x| {
            let character = x.chars().nth(position).unwrap();

            if retain(column_sum) {
                character == '1'
            } else {
                character == '0'
            }
        });

        position += 1;
    }

    usize::from_str_radix(&entries[0], 2).expect("Expected a binary entry for rating")
}

fn calculate_column_sum(report: DiagnosticReport, position: usize) -> isize {
    report
        .iter()
        .nth(position)
        .unwrap()
        .iter()
        .map(|v| match v {
            0 => -1,
            1 => 1,
            _ => panic!("Unexpected value"),
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        assert!(true)
    }

    #[test]
    fn sample1() {
        let input = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];

        assert_eq!(part1(&input).unwrap(), 198);
    }

    #[test]
    fn o2_rating() {
        let mut input = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];

        assert_eq!(calculate_rating(&mut input, |x| x >= 0), 23);
    }

    #[test]
    fn co2_rating() {
        let mut input = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];
        assert_eq!(calculate_rating(&mut input, |x| x < 0), 10);
    }
}
