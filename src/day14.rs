use aoc_runner_derive::{aoc, aoc_generator};
use radix_fmt;
use rustc_hash::FxHashMap;
use std::iter::repeat;

enum Version {
    V1,
    V2,
}

#[derive(Debug, Clone, Copy)]
struct Mask {
    zeroes: u64,
    ones: u64,
}

impl From<&str> for Mask {
    fn from(s: &str) -> Self {
        // They describe it as a "mask", but it's really two masks:
        let zeroes = !u64::from_str_radix(&format!("{:1>64}", s).replace('X', "1"), 2).unwrap();
        let ones = u64::from_str_radix(&format!("{:0>64}", s).replace('X', "0"), 2).unwrap();
        Self { zeroes, ones }
    }
}

impl Mask {
    fn apply_v1(&self, x: u64) -> u64 {
        x & !self.zeroes | self.ones
    }
    fn floating_mask(&self) -> u64 {
        // ones for all unset bits
        // Example:
        // original: xxx1x0xx00011;
        // ones:     0001000000011;
        // !ones:    1110111111100;
        // zeroes:   1111101100011;
        // output    1110101100000
        let floating = (!self.zeroes & !self.ones) & (!0 >> (64 - 36));
        floating
    }
    fn bits(x: usize) -> String {
        format!("{}", radix_fmt::radix(x, 2))
    }
    fn make_nth_mask(n: usize, floating_bits_indices: &[usize]) -> usize {
        let mask = floating_bits_indices
            .iter()
            .enumerate()
            .map(|(index, floating_bits_index)| {
                let bit_from_n = n >> index & 1;
                bit_from_n << floating_bits_index
            })
            .fold(0, |acc, x| acc | x);
        mask
    }
    fn make_all_masks(&self) -> impl Iterator<Item = usize> {
        let floating_bits = Self::bits(self.floating_mask() as usize);
        let floating_bits_indices = floating_bits
            .chars()
            .rev()
            .enumerate()
            .filter_map(|(index, bit)| if bit == '1' { Some(index) } else { None })
            .collect::<Vec<_>>();
        (0..2_usize.pow(floating_bits_indices.len() as u32))
            .map(move |n| Self::make_nth_mask(n, &floating_bits_indices))
    }
    fn apply_v2(&self, address: usize) -> Vec<usize> {
        let floating_mask = self.floating_mask() as usize;
        let ones = self.ones as usize;
        self.make_all_masks()
            .map(|mask| {
                let with_ones = address | ones;
                let with_cleared_floating_mask = with_ones & !floating_mask;
                let with_this_mask = with_cleared_floating_mask | mask;
                with_this_mask
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    UpdateMask(Mask),
    Write(usize, u64),
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        if s.starts_with("mem") {
            let (left, right) = s.split_once(" = ").unwrap();
            let address = left.replace("mem[", "").replace("]", "").parse().unwrap();
            let value = right.parse().unwrap();
            Self::Write(address, value)
        } else if s.starts_with("mask") {
            Self::UpdateMask(s.split(" = ").nth(1).unwrap().into())
        } else {
            panic!("Unknown operation: {}", s);
        }
    }
}

#[derive(Debug, Clone)]
struct Program {
    ops: Vec<Op>,
    mask: Mask,
    mem: FxHashMap<usize, u64>,
}

impl From<&str> for Program {
    fn from(s: &str) -> Self {
        let mask = repeat('X').take(36).collect::<String>().as_str().into();
        let ops = s
            .split_terminator('\n')
            .map(|line| line.into())
            .collect::<Vec<_>>();
        Self {
            mask,
            ops,
            mem: FxHashMap::default(),
        }
    }
}

impl Program {
    fn run(&mut self, version: Version) {
        for op in self.ops.clone().iter() {
            match version {
                Version::V1 => match op {
                    Op::UpdateMask(m) => self.mask = *m,
                    Op::Write(address, value) => {
                        *self.mem.entry(*address).or_insert(0) = self.mask.apply_v1(*value);
                    }
                },
                Version::V2 => match op {
                    Op::UpdateMask(m) => self.mask = *m,
                    Op::Write(address, value) => {
                        self.mask.apply_v2(*address).iter().for_each(|address| {
                            *self.mem.entry(*address).or_insert(0) = *value;
                        })
                    }
                },
            }
        }
    }
    fn mem_sum(&self) -> u64 {
        self.mem.values().sum()
    }
}

#[aoc_generator(day14)]
fn parse_input(s: &str) -> Program {
    s.into()
}

#[aoc(day14, part1)]
fn part1(prog: &Program) -> u64 {
    let mut prog = prog.clone();
    prog.run(Version::V1);
    prog.mem_sum()
}

#[aoc(day14, part2)]
fn part2(prog: &Program) -> u64 {
    let mut prog = prog.clone();
    prog.run(Version::V2);
    prog.mem_sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Program {
        parse_input(include_str!("../input/2020/day14.txt"))
    }
    #[test]
    fn test_mask() {
        let mask: Mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".into();
        assert_eq!(73, mask.apply_v1(11));
        assert_eq!(101, mask.apply_v1(101));
        assert_eq!(64, mask.apply_v1(0));
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 7440382076205)
    }
    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2(&parse_input(
                "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1"
            )),
            208
        )
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 4200656704538)
    }
}
