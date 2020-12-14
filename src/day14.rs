use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::repeat;

#[derive(Debug, Clone, Copy)]
struct Mask {
    and_mask: u64,
    or_mask: u64,
}

impl From<&str> for Mask {
    fn from(s: &str) -> Self {
        // They describe it as a "mask", but it's really two masks:
        // Apply bitwise and with all the ones (where X means 1)
        // Apply bitwise or with all the zeroes (where X means 0)
        let and_mask = u64::from_str_radix(&format!("{:1>64}", s).replace('X', "1"), 2).unwrap();
        let or_mask = u64::from_str_radix(&format!("{:0>64}", s).replace('X', "0"), 2).unwrap();
        Self { and_mask, or_mask }
    }
}

impl Mask {
    fn apply_to(&self, x: u64) -> u64 {
        x & self.and_mask | self.or_mask
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
    mem: Vec<u64>,
}

impl From<&str> for Program {
    fn from(s: &str) -> Self {
        let mask = repeat('X').take(36).collect::<String>().as_str().into();
        let ops = s
            .split_terminator('\n')
            .map(|line| line.into())
            .collect::<Vec<_>>();
        let max_address = ops
            .iter()
            .filter_map(|op| {
                if let Op::Write(address, _) = op {
                    Some(*address)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);
        Self {
            mask,
            ops,
            mem: repeat(0).take(max_address + 1).collect(),
        }
    }
}

impl Program {
    fn run(&mut self) {
        for op in self.ops.iter() {
            match op {
                Op::UpdateMask(m) => self.mask = *m,
                Op::Write(address, value) => self.mem[*address] = self.mask.apply_to(*value),
            }
        }
    }
    fn mem_sum(&self) -> u64 {
        self.mem.iter().sum()
    }
}

#[aoc_generator(day14)]
fn parse_input(s: &str) -> Program {
    s.into()
}

#[aoc(day14, part1)]
fn part1(prog: &Program) -> u64 {
    let mut prog = prog.clone();
    prog.run();
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
        assert_eq!(73, mask.apply_to(11));
        assert_eq!(101, mask.apply_to(101));
        assert_eq!(64, mask.apply_to(0));
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 7440382076205)
    }
}
