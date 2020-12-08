use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeSet;

#[aoc_generator(day6)]
fn parse_input(data: &str) -> Vec<Vec<String>> {
    data.split_terminator("\n\n")
        .map(|s| s.split_terminator('\n').map(|s| s.to_string()).collect())
        .collect()
}

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

#[aoc(day6, part1)]
fn part1(data: &[Vec<String>]) -> usize {
    data.iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|person| person.chars())
                // Benchmarked to be better than a HashSet here, and equivalent to a sorted and
                // deduplicated Vec
                .collect::<BTreeSet<_>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(data: &[Vec<String>]) -> usize {
    let mut chars_present: Vec<bool> = ALPHABET.chars().map(|_| true).collect();
    data.iter()
        .map(|group| {
            chars_present.iter_mut().for_each(|c| *c = true);
            for person in group.iter() {
                let mut chars = person.chars().collect::<Vec<_>>();
                chars.sort();
                chars.dedup();
                let mut chars_idx = 0;
                for (letter, char_present) in ALPHABET.chars().zip(chars_present.iter_mut()) {
                    if let Some(c) = chars.get(chars_idx) {
                        if letter != *c {
                            *char_present = false;
                        } else {
                            chars_idx += 1;
                        }
                    } else {
                        *char_present = false;
                    }
                }
            }
            chars_present.iter().filter(|b| **b).count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Vec<Vec<String>> {
        parse_input(include_str!("../input/2020/day6.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 6686)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 3476)
    }
}
