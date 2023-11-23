#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> [u64; 9] {
    let mut fish_counts = [0u64; 9];

    let population: Vec<u64> = input.split(",").map(|n| n.parse().unwrap()).collect();

    for f in population {
        fish_counts[f as usize] += 1;
    }

    fish_counts
}

#[aoc(day6, part1)]
pub fn part1(fish_counts: &[u64; 9]) -> u64 {
    let mut fish_counts = fish_counts.clone();

    for _ in 0..80 {
        simulate(&mut fish_counts);
    }

    fish_counts.iter().sum()
}

#[aoc(day6, part2)]
pub fn part2(fish_counts: &[u64; 9]) -> u64 {
    let mut fish_counts = fish_counts.clone();

    for _ in 0..256 {
        simulate(&mut fish_counts);
    }

    fish_counts.iter().sum()
}

fn simulate(fish_counts: &mut [u64]) {
    let reproducing = fish_counts[0];

    for i in 0..8 {
        fish_counts[i] = fish_counts[i + 1]
    }

    fish_counts[6] += reproducing;
    fish_counts[8] = reproducing;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let input = input_generator("1,2,3,3");

        assert_eq!(input, [0, 1, 1, 2, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn simulate_is_correct() {
        let mut fish_counts = [1, 1, 2, 3, 0, 0, 0, 0, 0];

        simulate(&mut fish_counts);

        assert_eq!(fish_counts, [1, 2, 3, 0, 0, 0, 1, 0, 1]);
    }
}
