use std::collections::HashSet;

fn parse_input() -> Vec<Vec<&'static str>> {
    let data = include_str!("input.txt");
    data.split_terminator("\n\n")
        .map(|s| s.split_terminator('\n').collect())
        .collect()
}

fn part1() -> usize {
    parse_input()
        .iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|person| person.chars())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

fn main() {
    println!("part 1: {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 6686)
    }
}
