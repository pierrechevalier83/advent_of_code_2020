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
    // Benchmarked to be significantly better than HashSet in this case
    // It's a small collection: 25 elements in the example: log(N) is smaller than the constant terms
    // incurred by a HashSet
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

struct Window<'a> {
    data: &'a [u64],
    start: usize,
    end: usize,
    min_size: usize,
}

impl<'a> Window<'a> {
    fn from_data_and_min_size(data: &'a [u64], min_size: usize) -> Self {
        Self {
            data,
            start: 0,
            end: min_size,
            min_size,
        }
    }
    fn front(&self) -> Option<u64> {
        self.data.get(self.start).copied()
    }
    fn back(&self) -> Option<u64> {
        self.data.get(self.end - 1).copied()
    }
    fn push_back(&mut self) -> Option<u64> {
        if self.end < self.data.len() {
            self.end += 1;
            self.back()
        } else {
            None
        }
    }
    fn pop_back(&mut self) -> Option<u64> {
        let falling_out = self.back();
        if self.start + self.min_size < self.end {
            self.end -= 1;
            falling_out
        } else {
            None
        }
    }
    fn pop_front(&mut self) -> Option<u64> {
        let falling_out = self.front();
        if self.start + self.min_size < self.end {
            self.start += 1;
            falling_out
        } else {
            None
        }
    }
    fn view(&self) -> impl Iterator<Item = u64> + 'a {
        self.data.iter().take(self.end).skip(self.start).copied()
    }
    fn at_right_edge(&self) -> bool {
        self.back().is_none()
    }
    fn at_min_size(&self) -> bool {
        self.end <= self.start + self.min_size
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
    fn find_window(&self, target_sum: u64) -> u64 {
        let mut window = Window::from_data_and_min_size(&self.data, 2);
        let mut sum: u64 = window.view().sum();
        while !window.at_right_edge() {
            if sum < target_sum {
                sum += window.push_back().unwrap();
            } else if sum > target_sum {
                sum -= window.pop_front().unwrap();
                while sum > target_sum && !window.at_min_size() {
                    sum -= window.pop_back().unwrap()
                }
            } else {
                return window.view().min().unwrap() + window.view().max().unwrap();
            }
        }
        panic!("We expected to find a window");
    }
}

#[aoc(day9, part1)]
fn part1(data: &Data) -> u64 {
    data.find_first_invalid_number()
}

#[aoc(day9, part2)]
fn part2(data: &Data) -> u64 {
    data.find_window(data.find_first_invalid_number())
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
        assert_eq!(62, part2(&data));
    }
    fn input() -> Data {
        parse_input(include_str!("../input/2020/day9.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 373803594)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 51152360)
    }
}
