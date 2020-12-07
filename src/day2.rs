use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::str::FromStr;

struct Policy {
    x: usize,
    y: usize,
    letter: char,
}

impl FromStr for Policy {
    type Err = String;

    // Parse a string with format: "10-12 k"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((range, letter)) = s.split(' ').collect_tuple() {
            if let Some((x, y)) = range.split('-').collect_tuple() {
                Ok(Self {
                    x: x.parse().map_err(|_| "Couldn't parse x".to_string())?,
                    y: y.parse().map_err(|_| "Couldn't parse y".to_string())?,
                    letter: letter
                        .parse()
                        .map_err(|_| "couldn't parse letter".to_string())?,
                })
            } else {
                return Err("Expected a single dash".to_string());
            }
        } else {
            return Err("Expected a single space".to_string());
        }
    }
}

impl Policy {
    // Does this password respect this policy?
    // (letter occurs at least x and up to y)
    fn allows_password_in_part_1(&self, password: &str) -> bool {
        let num_letters = password.chars().filter(|c| *c == self.letter).count();
        num_letters >= self.x && num_letters <= self.y
    }
    // Does this password respect this policy?
    // (letter occurs exactly once at one of these two one-based indices)
    fn allows_password_in_part_2(&self, password: &str) -> bool {
        (password.chars().nth(self.x - 1) == Some(self.letter))
            ^ (password.chars().nth(self.y - 1) == Some(self.letter))
    }
}

#[aoc_generator(day2)]
fn parse_input(data: &str) -> Vec<(Policy, String)> {
    data.split_terminator('\n')
        .filter_map(|s| s.split(": ").collect_tuple())
        .map(|(policy, password)| (Policy::from_str(policy).unwrap(), password.to_string()))
        .collect()
}

// Count the number of valid passwords in the file
#[aoc(day2, part1)]
fn part1(data: &[(Policy, String)]) -> usize {
    data.iter()
        .filter(|(policy, password)| policy.allows_password_in_part_1(&password))
        .count()
}

// Count the number of valid passwords in the file
#[aoc(day2, part2)]
fn part2(data: &[(Policy, String)]) -> usize {
    data.iter()
        .filter(|(policy, password)| policy.allows_password_in_part_2(&password))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Vec<(Policy, String)> {
        parse_input(include_str!("../input/2020/day2.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 517)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 284)
    }
}
