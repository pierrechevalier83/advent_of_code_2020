use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Jump(isize),
    Accumulate(i32),
    NoOp(isize),
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
            Some(("nop", x)) => Ok(Self::NoOp(
                x.parse::<isize>()
                    .map_err(|_| format!("Couldn't parse {}", x))?,
            )),
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
            Self::NoOp(_) => {
                *position += 1;
            }
        }
        *accumulator
    }
}

#[derive(Clone, Copy, Debug)]
enum ProgramStatus {
    Running,
    Terminated,
    Cycle,
}

#[derive(Clone)]
struct Program {
    lines: Vec<Instruction>,
    lines_visited: HashSet<usize>,
    accumulator: i32,
    position: usize,
    status: ProgramStatus,
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
            status: ProgramStatus::Running,
        })
    }
}

impl Iterator for Program {
    type Item = Program;
    fn next(&mut self) -> Option<Self::Item> {
        match self.status {
            ProgramStatus::Running => {
                if self.position >= self.lines.len() {
                    self.status = ProgramStatus::Terminated;
                } else if !self.lines_visited.insert(self.position) {
                    self.status = ProgramStatus::Cycle;
                } else {
                    self.execute_instruction();
                }
                Some(self.clone())
            }
            _ => None,
        }
    }
}

impl Program {
    fn execute_instruction(&mut self) {
        self.lines[self.position].execute(&mut self.accumulator, &mut self.position);
    }
    fn permut_line(&mut self, index: usize) -> bool {
        self.lines
            .get_mut(index)
            .map(|line| match line {
                Instruction::NoOp(x) => {
                    *line = Instruction::Jump(*x);
                    true
                }
                Instruction::Jump(x) => {
                    *line = Instruction::NoOp(*x);
                    true
                }
                _ => false,
            })
            .unwrap_or(false)
    }
}

struct ProgramPermutations {
    program: Program,
    permuted_line: usize,
}

impl Iterator for ProgramPermutations {
    type Item = Program;
    fn next(&mut self) -> Option<Self::Item> {
        let mut program = self.program.clone();
        loop {
            if program.permut_line(self.permuted_line) {
                self.permuted_line += 1;
                return Some(program);
            } else {
                if self.permuted_line >= self.program.lines.len() {
                    return None;
                } else {
                    self.permuted_line += 1;
                }
            }
        }
    }
}

#[aoc_generator(day8)]
fn parse_input(data: &str) -> Program {
    Program::from_str(data).unwrap()
}

#[aoc(day8, part1)]
fn part1(program: &Program) -> i32 {
    program.clone().last().unwrap().accumulator
}

#[aoc(day8, part2)]
fn part2(program: &Program) -> i32 {
    ProgramPermutations {
        program: program.clone(),
        permuted_line: 0,
    }
    .find_map(|program| {
        let program = program.last().unwrap();
        let status = program.status;
        match status {
            ProgramStatus::Terminated => Some(program.accumulator),
            _ => None,
        }
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    use super::*;
    fn input() -> Program {
        parse_input(include_str!("../input/2020/day8.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 1487)
    }
    #[test]
    fn test_example_part_2() {
        assert_eq!(8, part2(&parse_input(EXAMPLE_INPUT)));
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1607)
    }
}
