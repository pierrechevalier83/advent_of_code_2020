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

fn all_joltages(adapters: &[u8]) -> Vec<u8> {
    let mut joltages = adapters.to_vec();
    joltages.push(0);
    joltages.push(device_built_in_adapter(&adapters));
    joltages.sort();
    joltages
}

fn diffs(adapters: &[u8]) -> Vec<u8> {
    all_joltages(adapters)
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

fn count_diffs_of_1_and_3(adapters: &[u8]) -> (usize, usize) {
    let diffs = diffs(adapters);
    (
        diffs.iter().filter(|diff| **diff == 1).count(),
        diffs.iter().filter(|diff| **diff == 3).count(),
    )
}

#[aoc(day10, part1)]
fn part1(adapters: &[u8]) -> usize {
    let (diffs_of_1, diffs_of_3) = count_diffs_of_1_and_3(adapters);
    diffs_of_1 * diffs_of_3
}

fn count_neighbours(index: usize, joltages: &[u8]) -> usize {
    joltages
        .iter()
        .skip(index + 1)
        .take(3)
        .filter(|x| (*x - joltages[index]) <= 3)
        .count()
}

fn num_paths_from_index(index: usize, joltages: &[u8], visited: &mut Vec<Option<usize>>) -> usize {
    if index == joltages.len() - 1 {
        1
    } else if let Some(num_paths) = visited[index] {
        num_paths
    } else {
        let num_paths = (index + 1..index + count_neighbours(index, joltages) + 1)
            .map(|index| num_paths_from_index(index, joltages, visited))
            .sum();
        visited[index] = Some(num_paths);
        num_paths
    }
}

#[aoc(day10, part2)]
fn part2(adapters: &[u8]) -> usize {
    let all_joltages = all_joltages(adapters);
    let mut visited = all_joltages.iter().map(|_| None).collect();
    num_paths_from_index(0, &all_joltages, &mut visited)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Vec<u8> {
        parse_input(include_str!("../input/2020/day10.txt"))
    }
    #[test]
    fn test_small_example() {
        assert_eq!(35, part1(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]));
        assert_eq!(8, part2(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 3034)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 259172170858496)
    }
}
