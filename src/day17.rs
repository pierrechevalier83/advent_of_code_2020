use crate::point3d::Point;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct ConwayCube {
    active: HashSet<Point>,
}

impl From<&str> for ConwayCube {
    fn from(s: &str) -> Self {
        Self {
            active: s
                .split("\n")
                .enumerate()
                .flat_map(|(row_index, row)| {
                    row.chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '#')
                        .map(move |(col_index, _)| {
                            Point::new(row_index as isize, col_index as isize, 0)
                        })
                })
                .collect(),
        }
    }
}

impl ConwayCube {
    fn remains_active(&self, p: Point) -> bool {
        let num_active_neighbours = p.neighbours().filter(|n| self.active.contains(n)).count();
        num_active_neighbours == 2 || num_active_neighbours == 3
    }
    fn becomes_active(&self, p: Point) -> bool {
        p.neighbours().filter(|n| self.active.contains(n)).count() == 3
    }
    fn next(self) -> Self {
        Self {
            active: self
                .active
                .iter()
                .copied()
                .filter(|p| self.remains_active(*p))
                .chain(
                    self.active
                        .iter()
                        .flat_map(|p| p.neighbours())
                        .filter(|p| !self.active.contains(p) && self.becomes_active(*p)),
                )
                .collect(),
        }
    }
    fn nth(self, n: usize) -> Self {
        let mut nth = self;
        for _ in 0..n {
            nth = nth.next();
        }
        nth
    }
    fn num_active(&self) -> usize {
        self.active.len()
    }
}

#[aoc_generator(day17)]
fn parse_input(s: &str) -> ConwayCube {
    let cube = ConwayCube::from(s);
    cube
}

#[aoc(day17, part1)]
fn part1(cube: &ConwayCube) -> usize {
    cube.clone().nth(6).num_active()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> ConwayCube {
        parse_input(include_str!("../input/2020/day17.txt"))
    }
    #[test]
    fn test_example() {
        let input = ".#.\n..#\n###";
        assert_eq!(112, ConwayCube::from(input).nth(6).num_active())
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 42)
    }
}
