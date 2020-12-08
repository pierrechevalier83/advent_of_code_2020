use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Instruction {
    Jump(isize),
    Accumulate(i32),
    NoOp,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect_tuple() {
            Some(("jmp", x)) => Ok(Self::Jump(
                x.parse::<isize>()
                    .map_err(|_| format!("Couldn't parse {}", x))?,
            )),
            Some(("acc", x)) => Ok(Self::Accumulate(
                x.parse::<i32>()
                    .map_err(|_| format!("Couldn't parse {}", x))?,
            )),
            Some(("nop", _)) => Ok(Self::NoOp),
            _ => Err(format!("Invalid instruction: {}", s)),
        }
    }
}

impl Instruction {
    fn execute(self, accumulator: &mut i32, position: &mut usize) -> i32 {
        match self {
            Self::Jump(x) => *position = (*position as isize + x) as usize,
            Self::Accumulate(x) => {
                *position += 1;
                *accumulator += x
            }
            Self::NoOp => {
                *position += 1;
            }
        }
        *accumulator
    }
}

#[derive(Clone)]
struct Program {
    lines: Vec<Instruction>,
    lines_visited: HashSet<usize>,
    accumulator: i32,
    position: usize,
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            lines: s
                .split_terminator('\n')
                .map(|line| Instruction::from_str(line))
                .collect::<Result<Vec<_>, _>>()?,
            lines_visited: HashSet::new(),
            accumulator: 0,
            position: 0,
        })
    }
}

impl Iterator for Program {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.lines_visited.insert(self.position) {
            None
        } else {
            Some(self.execute_instruction())
        }
    }
}

impl Program {
    fn execute_instruction(&mut self) -> i32 {
        let instruction = self.lines[self.position];
        instruction.execute(&mut self.accumulator, &mut self.position)
    }
}

#[aoc_generator(day8)]
fn parse_input(data: &str) -> Program {
    Program::from_str(data).unwrap()
}

#[aoc(day8, part1)]
fn part1(program: &Program) -> i32 {
    program.clone().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Program {
        parse_input(include_str!("../input/2020/day8.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 1487)
    }
}
