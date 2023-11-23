#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(",").map(|s| s.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn part1(positions: &[i32]) -> i32 {
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();

    (*min..*max)
        .map(|i| positions.iter().map(|p| (i - p).abs()).sum())
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn part2(positions: &[i32]) -> i32 {
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();

    (*min..*max)
        .map(|i| positions.iter().map(|p| fuel((i - p).abs())).sum())
        .min()
        .unwrap()
}

fn fuel(d: i32) -> i32 {
    // use arithmetic series
    // https://mathworld.wolfram.com/ArithmeticSeries.html
    d * (1 + d) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_test() {
        assert_eq!(fuel(4), 10);
    }
}
