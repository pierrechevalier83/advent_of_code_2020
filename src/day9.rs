use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeSet;

#[aoc_generator(day9)]
fn parse_input(data: &str) -> Data {
    Data::from((
        data.split_terminator('\n')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>(),
        25,
    ))
}

struct Preamble {
    numbers: BTreeSet<u64>,
    start_index: usize,
}

impl Preamble {
    fn slide(&mut self, data: &[u64]) {
        let size = self.numbers.len();
        self.numbers.remove(&data[self.start_index]);
        self.numbers.insert(data[self.start_index + size]);
        self.start_index += 1;
    }
    fn is_valid_next(&self, target: u64) -> bool {
        self.numbers.iter().any(|x| {
            *x != target
                && self
                    .numbers
                    .contains(&((target as i64 - *x as i64).abs() as u64))
        })
    }
}

impl From<BTreeSet<u64>> for Preamble {
    fn from(numbers: BTreeSet<u64>) -> Self {
        Self {
            numbers,
            start_index: 0,
        }
    }
}
struct Data {
    preamble_size: usize,
    data: Vec<u64>,
}

impl From<(Vec<u64>, usize)> for Data {
    fn from(input: (Vec<u64>, usize)) -> Self {
        let (data, preamble_size) = input;
        Self {
            preamble_size,
            data,
        }
    }
}

impl Data {
    fn make_preamble(&self) -> Preamble {
        Preamble::from(
            self.data
                .iter()
                .take(self.preamble_size)
                .copied()
                .collect::<BTreeSet<u64>>(),
        )
    }
    fn find_first_invalid_number(&self) -> u64 {
        let mut preamble = self.make_preamble();
        self.data
            .iter()
            .skip(self.preamble_size)
            .find(|x| {
                if !preamble.is_valid_next(**x) {
                    true
                } else {
                    preamble.slide(&self.data);
                    false
                }
            })
            .copied()
            .unwrap()
    }
}

#[aoc(day9, part1)]
fn part1(data: &Data) -> u64 {
    data.find_first_invalid_number()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let data = Data::from((
            vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ],
            5,
        ));
        assert_eq!(127, part1(&data));
    }
    fn input() -> Data {
        parse_input(include_str!("../input/2020/day9.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 373803594)
    }
}
