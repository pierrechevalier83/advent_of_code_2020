use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(data: &str) -> Vec<u32> {
    let mut data = data
        .split_terminator('\n')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    // Sort the data in O(n*log(n))
    data.sort();
    data
}

const TARGET: u32 = 2020;

// Find two entries that add up to 2020 and return their product
// Complexity: O(n*log(n))
#[aoc(day1, part1)]
fn part1(data: &[u32]) -> u32 {
    // Then find any datum with a complement in the data and return the product
    data.iter()
        .filter(|datum| *datum <= &TARGET)
        .find_map(|datum| {
            let complement = TARGET - datum;
            data.binary_search(&complement)
                .ok()
                .map(|_| datum * complement)
        })
        .unwrap()
}

// Find three entries that add up to 2020 and return their product
// Complexity: O(n^2*log(n))
#[aoc(day1, part2)]
fn part2(data: &[u32]) -> u32 {
    data
        // Iterate on the first level
        .iter()
        .enumerate()
        .flat_map(|(index, first)| {
            // Iterate again, skipping pairs we've already considered
            data.iter().skip(index).map(move |second| (*first, *second))
        })
        .filter(|(first, second)| first + second <= TARGET)
        .find_map(|(first, second)| {
            let third = TARGET - (first + second);
            // Find the complement with a binary_search
            data.binary_search(&third)
                .ok()
                .map(|_| first * second * third)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<u32> {
        parse_input(include_str!("../input/2020/day1.txt"))
    }
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 719796)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 144554112)
    }
}
