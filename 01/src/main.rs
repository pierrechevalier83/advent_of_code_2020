fn parse_input() -> Vec<u32> {
    let data = include_str!("input.txt");
    data.split_terminator('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

// Find two entries that add up to 2020 and return their product
// Complexity: O(n*log(n))
fn part1() -> Option<u32> {
    let mut data = parse_input();
    // Sort the data in O(n*log(n))
    data.sort();
    let view = &data;
    // Then find any datum with a complement in the data and return the product
    data.iter()
        .filter_map(|datum| {
            if *datum <= 2020 {
                let complement = 2020 - datum;
                view.binary_search(&complement)
                    .ok()
                    .map(|_| datum * complement)
            } else {
                None
            }
        })
        .next()
}

// Find three entries that add up to 2020 and return their product
// Complexity: O(n^2*log(n))
fn part2() -> Option<u32> {
    let mut data = parse_input();
    // Sort the data in O(n*log(n))
    data.sort();
    let view = &data;
    // Iterate on the first level
    data.iter()
        .enumerate()
        .flat_map(|(index, first)| {
            // Iterate again, skipping pairs we've already considered
            data.iter().skip(index).map(move |second| (*first, *second))
        })
        .filter_map(|(first, second)| {
            if first + second <= 2020 {
                let third = 2020 - (first + second);
                // Find the complement with a binary_search
                view.binary_search(&third)
                    .ok()
                    .map(|_| first * second * third)
            } else {
                None
            }
        })
        .next()
}

fn main() {
    println!(
        "part 1: {}",
        part1().expect("We assumed the input contained at least one pair")
    );
    println!(
        "part 2: {}",
        part2().expect("We assumed the input contained at least one triplet")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), Some(719796))
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), Some(144554112))
    }
}
