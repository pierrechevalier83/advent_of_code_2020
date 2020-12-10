use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(data: &str) -> Vec<u8> {
    data.split_terminator('\n')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}

fn device_built_in_adapter(data: &[u8]) -> u8 {
    data.iter().max().unwrap() + 3
}

fn count_diffs_of_1_and_3(mut adapters: Vec<u8>) -> (usize, usize) {
    adapters.push(0);
    adapters.push(device_built_in_adapter(&adapters));
    adapters.sort();
    let diffs = adapters.windows(2).map(|window| window[1] - window[0]);
    (
        diffs.clone().filter(|diff| *diff == 1).count(),
        diffs.filter(|diff| *diff == 3).count(),
    )
}

#[aoc(day10, part1)]
fn part1(adapters: &[u8]) -> usize {
    let (diffs_of_1, diffs_of_3) = count_diffs_of_1_and_3(adapters.to_vec());
    println!("({}, {})", diffs_of_1, diffs_of_3);
    diffs_of_1 * diffs_of_3
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Vec<u8> {
        parse_input(include_str!("../input/2020/day10.txt"))
    }
    #[test]
    fn test_small_example() {
        assert_eq!(35, part1(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 3034)
    }
}
