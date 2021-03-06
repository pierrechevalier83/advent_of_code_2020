use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
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
    fn execute(self, accumulator: &mut i32, position: &mut usize, permuted: bool) -> i32 {
        match self {
            Self::Jump(x) => {
                if permuted {
                    *position += 1;
                } else {
                    *position = (*position as isize + x) as usize;
                }
            }
            Self::Accumulate(x) => {
                *position += 1;
                *accumulator += x;
            }
            Self::NoOp(x) => {
                if permuted {
                    *position = (*position as isize + x) as usize;
                } else {
                    *position += 1;
                }
            }
        };
        *accumulator
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProgramStatus {
    Running,
    Terminated,
    Cycle,
}

#[derive(Clone)]
struct Program<'a> {
    instructions: &'a [Instruction],
    lines_visited: Vec<bool>,
    accumulator: i32,
    position: usize,
    permutation: Option<usize>,
    status: ProgramStatus,
}

impl<'a> Program<'a> {
    fn new(instructions: &'a [Instruction], permutation: Option<usize>) -> Self {
        Self {
            instructions,
            lines_visited: (0..instructions.len()).map(|_| false).collect(),
            accumulator: 0,
            position: 0,
            permutation,
            status: ProgramStatus::Running,
        }
    }
    fn execute_instruction(&mut self) {
        let permuted = self.permutation == Some(self.position);
        self.lines_visited[self.position] = true;
        self.instructions[self.position].execute(
            &mut self.accumulator,
            &mut self.position,
            permuted,
        );
    }
    fn next_instruction(&mut self) {
        if self.position >= self.instructions.len() {
            self.status = ProgramStatus::Terminated;
        } else if self.lines_visited[self.position] {
            self.status = ProgramStatus::Cycle;
        } else {
            self.execute_instruction();
        }
    }
    fn run(&mut self) -> (i32, ProgramStatus) {
        while self.status != ProgramStatus::Terminated && self.status != ProgramStatus::Cycle {
            self.next_instruction();
        }
        (self.accumulator, self.status)
    }
    fn would_permut_line(&self, index: usize) -> bool {
        self.instructions
            .get(index)
            .map(|line| match line {
                Instruction::NoOp(_) | Instruction::Jump(_) => true,
                _ => false,
            })
            .unwrap_or(false)
    }
}

struct ProgramPermutations<'a> {
    instructions: &'a [Instruction],
    permuted_line: usize,
}

impl<'a> Iterator for ProgramPermutations<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let program = Program::new(self.instructions, None);
        loop {
            if program.would_permut_line(self.permuted_line) {
                let permutation = Some(self.permuted_line);
                self.permuted_line += 1;
                return permutation;
            } else {
                if self.permuted_line >= self.instructions.len() {
                    return None;
                } else {
                    self.permuted_line += 1;
                }
            }
        }
    }
}

#[aoc_generator(day8)]
fn parse_input(data: &str) -> Vec<Instruction> {
    data.split_terminator('\n')
        .map(|line| Instruction::from_str(line))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[aoc(day8, part1)]
fn part1(instructions: &[Instruction]) -> i32 {
    Program::new(instructions, None).run().0
}

#[aoc(day8, part2)]
fn part2(instructions: &[Instruction]) -> i32 {
    ProgramPermutations {
        instructions,
        permuted_line: 0,
    }
    .find_map(|permutation| {
        let (accumulator, status) = Program::new(instructions, Some(permutation)).run();
        match status {
            ProgramStatus::Terminated => Some(accumulator),
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
    fn input() -> Vec<Instruction> {
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
