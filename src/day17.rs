use crate::{point3d, point4d, pointnd::PointND};
use aoc_runner_derive::aoc;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
struct ConwayCube<Point> {
    active: FxHashSet<Point>,
    inactive: FxHashSet<Point>,
}

impl<Point: PointND> From<&str> for ConwayCube<Point> {
    fn from(s: &str) -> Self {
        let active = s
            .split("\n")
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(col_index, _)| {
                        Point::from_2d(row_index as isize, col_index as isize)
                    })
            })
            .collect::<FxHashSet<_>>();
        let inactive = Self::inactive_from_active(&active);
        Self { active, inactive }
    }
}

impl<Point: PointND> ConwayCube<Point> {
    fn remains_active(&self, p: Point) -> bool {
        let num_active_neighbours = p
            .neighbours()
            .iter()
            .filter(|n| self.active.contains(n))
            .count();
        num_active_neighbours == 2 || num_active_neighbours == 3
    }
    fn becomes_active(&self, p: Point) -> bool {
        p.neighbours()
            .iter()
            .filter(|n| self.active.contains(n))
            .count()
            == 3
    }
    fn inactive_from_active(active: &FxHashSet<Point>) -> FxHashSet<Point> {
        active
            .iter()
            .flat_map(|p| p.neighbours())
            .filter(|p| !active.contains(p))
            .collect()
    }
    fn next(self) -> Self {
        let active = self
            .active
            .iter()
            .copied()
            .filter(|p| self.remains_active(*p))
            .chain(
                self.inactive
                    .iter()
                    .filter(|p| self.becomes_active(**p))
                    .copied(),
            )
            .collect();
        let inactive = Self::inactive_from_active(&active);
        Self { active, inactive }
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

#[aoc(day17, part1)]
fn part1(s: &str) -> usize {
    ConwayCube::<point3d::Point>::from(s).nth(6).num_active()
}

#[aoc(day17, part2)]
fn part2(s: &str) -> usize {
    ConwayCube::<point4d::Point>::from(s).nth(6).num_active()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> &'static str {
        include_str!("../input/2020/day17.txt")
    }
    #[test]
    fn test_example() {
        let input = ".#.\n..#\n###";
        assert_eq!(
            112,
            ConwayCube::<point3d::Point>::from(input)
                .nth(6)
                .num_active()
        );
        assert_eq!(
            848,
            ConwayCube::<point4d::Point>::from(input)
                .nth(6)
                .num_active()
        )
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 273)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1504)
    }
}
