/*
--- Day 6: Tuning Trouble ---
The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the star fruit grove.

As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says that it has many fancy features, but the most important one to set up right now is the communication system.

However, because he's heard you have significant experience dealing with signal-based systems, he convinced the other Elves that it would be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.

As if inspired by comedic timing, the device emits a few colorful sparks.

To be able to communicate with the Elves, the device needs to lock on to their signal. The signal is a series of seemingly-random characters that the device receives one at a time.

To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream. In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.

The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

For example, suppose you receive the following datastream buffer:

mjqjpqmgbljsphdztnvjfqwrcgsmlb
After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker. The first time a marker could occur is after the fourth character is received, making the most recent four characters mjqj. Because j is repeated, this isn't a marker.

The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are jpqm, which are all different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after 7 characters have been processed.

Here are a few more examples:

bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
How many characters need to be processed before the first start-of-packet marker is detected?


--- Part Two ---
Your device's communication system is correctly detecting packets, but still isn't working. It looks like it also needs to look for messages.

A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.

Here are the first positions of start-of-message markers for all of the above examples:

mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26
How many characters need to be processed before the first start-of-message marker is detected?
*/

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<char>) -> usize {
    find_marker(input, 4).expect("Could not find signal start!")
}

#[aoc(day6, part2)]
pub fn part2(input: &Vec<char>) -> usize {
    find_marker(input, 14).expect("Could not find signal start!")
}

fn find_marker(chars: &Vec<char>, count: usize) -> Option<usize> {
    chars.windows(count).enumerate().find_map(|(i, chars)| {
        let mut seen = [false; 52]; // 🤷 52 should be fine, right?

        for c in chars {
            let ci = (*c as usize) - 'Z' as usize;

            if seen[ci] {
                return None;
            }

            seen[ci] = true;
        }

        Some(i + count)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(
            input_generator(input),
            [
                'b', 'v', 'w', 'b', 'j', 'p', 'l', 'b', 'g', 'v', 'b', 'h', 's', 'r', 'l', 'p',
                'g', 'd', 'm', 'j', 'q', 'w', 'f', 't', 'v', 'n', 'c', 'z'
            ]
        );
    }

    #[test]
    fn sample1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part1(&input_generator(input)), 5);
    }

    #[test]
    fn sample2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part1(&input_generator(input)), 6);
    }

    #[test]
    fn sample3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part1(&input_generator(input)), 10);
    }

    #[test]
    fn sample4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part1(&input_generator(input)), 11);
    }

    #[test]
    fn sample5() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part2(&input_generator(input)), 19);
    }
    #[test]
    fn sample6() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part2(&input_generator(input)), 23);
    }

    #[test]
    fn sample7() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part2(&input_generator(input)), 23);
    }

    #[test]
    fn sample8() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part2(&input_generator(input)), 29);
    }

    #[test]
    fn sample9() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part2(&input_generator(input)), 26);
    }
}
