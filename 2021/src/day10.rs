#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split("\n").map(|s| s.to_string()).collect()
}

#[aoc(day10, part1)]
pub fn part1(lines: &[String]) -> usize {
    lines
        .iter()
        .map(analyze)
        .filter_map(AnalyzedLine::corrupted)
        .map(score_symbol)
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(lines: &[String]) -> usize {
    let mut scores: Vec<_> = lines
        .iter()
        .map(analyze)
        .filter_map(AnalyzedLine::incomplete)
        .map(completion_string)
        .map(score_completion)
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[derive(Debug, PartialEq)]
enum AnalyzedLine {
    Incomplete(Vec<char>),
    Corrupted(char),
    Complete,
}

impl AnalyzedLine {
    fn incomplete(self) -> Option<Vec<char>> {
        match self {
            AnalyzedLine::Incomplete(stack) => Some(stack),
            _ => None,
        }
    }

    fn corrupted(self) -> Option<char> {
        match self {
            AnalyzedLine::Corrupted(last_char) => Some(last_char),
            _ => None,
        }
    }
}

fn analyze(line: &String) -> AnalyzedLine {
    let mut stack = Vec::with_capacity(line.len() / 2);
    let mut chars = line.chars();

    while let Some(symbol) = chars.next() {
        if is_closing_symbol(symbol) {
            match stack.pop() {
                Some(b) if symbol == matching_closing_symbol(b) => (),
                Some(_) => return AnalyzedLine::Corrupted(symbol),
                None => return AnalyzedLine::Incomplete(stack),
            }
        } else {
            stack.push(symbol)
        }
    }

    if stack.len() > 0 {
        AnalyzedLine::Incomplete(stack)
    } else {
        AnalyzedLine::Complete
    }
}

fn completion_string(chars: Vec<char>) -> String {
    let mut completion = String::with_capacity(chars.len());

    for c in chars.iter().rev() {
        match c {
            '(' => completion.push_str(")"),
            '[' => completion.push_str("]"),
            '{' => completion.push_str("}"),
            '<' => completion.push_str(">"),
            _ => panic!("Unknown completion string for {}", c),
        }
    }

    completion
}

fn score_completion(completion: String) -> usize {
    completion.chars().fold(0, |acc, c| match c {
        ')' => acc * 5 + 1,
        ']' => acc * 5 + 2,
        '}' => acc * 5 + 3,
        '>' => acc * 5 + 4,
        _ => panic!("Unknown completion string for {}", c),
    })
}

fn score_symbol(symbol: char) -> usize {
    match symbol {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("No score for {}", symbol),
    }
}

fn is_closing_symbol(symbol: char) -> bool {
    symbol == ')' || symbol == ']' || symbol == '}' || symbol == '>'
}

fn matching_closing_symbol(symbol: char) -> char {
    match symbol {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("No matching closing symbol for {}", symbol),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        assert_eq!(input_generator("[{(\n(({[}{"), vec!["[{(", "(({[}{"]);
    }

    #[test]
    fn analysis() {
        assert_eq!(
            analyze(&String::from("{([(<{}[<>[]}>{[]{[(<()>")),
            AnalyzedLine::Corrupted('}')
        );
        assert_eq!(
            analyze(&String::from("[({(<(())[]>[[{[]{<()<>>")),
            AnalyzedLine::Incomplete(vec!['[', '(', '{', '(', '[', '[', '{', '{'])
        );
    }

    #[test]
    fn completion() {
        assert_eq!(completion_string(vec!['(', '[', '{', '<']), ">}])");
    }

    #[test]
    fn completion_score() {
        assert_eq!(score_completion(String::from("}}]])})]")), 288957);
        assert_eq!(score_completion(String::from(")}>]})")), 5566);
        assert_eq!(score_completion(String::from("}}>}>))))")), 1480781);
        assert_eq!(score_completion(String::from("]]}}]}]}>")), 995444);
        assert_eq!(score_completion(String::from("])}>")), 294);
    }

    #[test]
    fn symbol_score() {
        assert_eq!(score_symbol(')'), 3);
        assert_eq!(score_symbol(']'), 57);
        assert_eq!(score_symbol('}'), 1197);
        assert_eq!(score_symbol('>'), 25137);
    }
}
