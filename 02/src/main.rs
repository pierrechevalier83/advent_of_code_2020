use itertools::Itertools;
use std::str::FromStr;

struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

impl FromStr for Policy {
    type Err = String;

    // Parse a string with format: "10-12 k"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((range, letter)) = s.split(' ').collect_tuple() {
            if let Some((min, max)) = range.split('-').collect_tuple() {
                Ok(Self {
                    min: min.parse().map_err(|_| "Couldn't parse min".to_string())?,
                    max: max.parse().map_err(|_| "Couldn't parse max".to_string())?,
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
    fn allows_password(&self, password: &str) -> bool {
        let num_letters = password.chars().filter(|c| *c == self.letter).count();
        num_letters >= self.min && num_letters <= self.max
    }
}

fn parse_input() -> Vec<(Policy, String)> {
    let data = include_str!("input.txt");
    data.split_terminator('\n')
        .filter_map(|s| s.split(": ").collect_tuple())
        .map(|(policy, password)| (Policy::from_str(policy).unwrap(), password.to_string()))
        .collect()
}

// Count the number of valid passwords in the file
fn part1() -> usize {
    parse_input()
        .iter()
        .filter(|(policy, password)| policy.allows_password(&password))
        .count()
}

fn main() {
    println!("part 1: {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 517)
    }
}
