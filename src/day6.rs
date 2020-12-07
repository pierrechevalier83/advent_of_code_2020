use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day6)]
fn parse_input(data: &str) -> Vec<Vec<String>> {
    data.split_terminator("\n\n")
        .map(|s| s.split_terminator('\n').map(|s| s.to_string()).collect())
        .collect()
}

#[aoc(day6, part1)]
fn part1(data: &[Vec<String>]) -> usize {
    data.iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|person| person.chars())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(data: &[Vec<String>]) -> usize {
    data.iter()
        .map(|group| {
            group
                .iter()
                .map(|person| person.chars().collect::<HashSet<_>>())
                .fold_first(|a, b| a.intersection(&b).cloned().collect())
                .unwrap()
                .len()
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
