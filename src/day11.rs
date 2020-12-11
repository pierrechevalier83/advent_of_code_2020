use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::fmt::{self, Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Seat {
    Floor,
    Occupied,
    Empty,
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => panic!("Invalid input"),
        }
    }
}

impl Debug for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Seat::Floor => "██",
            Seat::Occupied => "🧘",
            Seat::Empty => "  ",
        };
        write!(f, "{}", c)
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Plane {
    seats: Vec<Vec<Seat>>,
}

impl Debug for Plane {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        for row in self.seats.iter() {
            for seat in row {
                write!(f, "{:?}", seat)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
impl From<&str> for Plane {
    fn from(s: &str) -> Self {
        Plane {
            seats: s
                .split_terminator('\n')
                .map(|s| s.chars().map(|c| c.into()).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }
}

impl Plane {
    fn all_directions() -> Vec<(isize, isize)> {
        let flat = [-1, 0, 1];
        flat.iter()
            .copied()
            .cartesian_product(flat.iter().copied())
            .filter(|pos| *pos != (0, 0))
            .collect()
    }
    fn adjacent_index(index: usize, dir: isize, len: usize) -> Option<usize> {
        if index == 0 && dir == -1 || index == len - 1 && dir == 1 {
            None
        } else {
            Some((index as isize + dir) as usize)
        }
    }
    fn adjacent_position(
        &self,
        pos: (usize, usize),
        dir: (isize, isize),
    ) -> Option<(usize, usize)> {
        Self::adjacent_index(pos.0, dir.0, self.n_rows())
            .and_then(|row| Self::adjacent_index(pos.1, dir.1, self.n_cols()).map(|col| (row, col)))
    }
    fn n_rows(&self) -> usize {
        self.seats.len()
    }
    fn n_cols(&self) -> usize {
        self.seats[0].len()
    }
    fn seat_at(&self, pos: (usize, usize)) -> Seat {
        self.seats[pos.0][pos.1]
    }
    fn n_occupied_seats(&self) -> usize {
        self.seats
            .iter()
            .flat_map(|row| row.iter())
            .filter(|s| **s == Seat::Occupied)
            .count()
    }
    fn adjacent_seats(&self, pos: (usize, usize)) -> Vec<Seat> {
        Self::all_directions()
            .iter()
            .filter_map(|dir| self.adjacent_position(pos, *dir))
            .map(move |pos| self.seat_at(pos))
            .collect()
    }
    fn updated_seat_part1(&self, pos: (usize, usize)) -> Seat {
        if self.seat_at(pos) == Seat::Empty
            && !self
                .adjacent_seats(pos)
                .iter()
                .any(|s| *s == Seat::Occupied)
        {
            Seat::Occupied
        } else if self.seat_at(pos) == Seat::Occupied
            && self
                .adjacent_seats(pos)
                .iter()
                .filter(|s| **s == Seat::Occupied)
                .count()
                >= 4
        {
            Seat::Empty
        } else {
            self.seat_at(pos)
        }
    }
    fn next_part1(&self) -> Option<Self> {
        let next = Self {
            seats: (0..self.n_rows())
                .map(|row| {
                    (0..self.n_cols())
                        .map(|col| self.updated_seat_part1((row, col)))
                        .collect()
                })
                .collect(),
        };
        if next != *self {
            Some(next)
        } else {
            None
        }
    }
    fn iterate_until_stable_part1(&self) -> Self {
        let mut prev = self.clone();
        loop {
            let next = prev.next_part1();
            if next.is_none() {
                return prev;
            }
            prev = next.unwrap();
        }
    }
    fn visible_position(&self, pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
        let mut pos = Some(pos);
        loop {
            pos = self.adjacent_position(pos.unwrap(), dir);
            if pos.is_none() {
                return None;
            } else if self.seat_at(pos.unwrap()) != Seat::Floor {
                return Some(pos.unwrap());
            }
        }
    }
    fn visible_seats(&self, pos: (usize, usize)) -> Vec<Seat> {
        Self::all_directions()
            .iter()
            .filter_map(|dir| self.visible_position(pos, *dir))
            .map(move |pos| self.seat_at(pos))
            .collect()
    }
    fn updated_seat_part2(&self, pos: (usize, usize)) -> Seat {
        if self.seat_at(pos) == Seat::Empty
            && !self.visible_seats(pos).iter().any(|s| *s == Seat::Occupied)
        {
            Seat::Occupied
        } else if self.seat_at(pos) == Seat::Occupied
            && self
                .visible_seats(pos)
                .iter()
                .filter(|s| **s == Seat::Occupied)
                .count()
                >= 5
        {
            Seat::Empty
        } else {
            self.seat_at(pos)
        }
    }
    fn next_part2(&self) -> Option<Self> {
        let next = Self {
            seats: (0..self.n_rows())
                .map(|row| {
                    (0..self.n_cols())
                        .map(|col| self.updated_seat_part2((row, col)))
                        .collect()
                })
                .collect(),
        };
        if next != *self {
            Some(next)
        } else {
            None
        }
    }
    fn iterate_until_stable_part2(&self) -> Self {
        let mut prev = self.clone();
        loop {
            let next = prev.next_part2();
            if next.is_none() {
                return prev;
            }
            prev = next.unwrap();
        }
    }
}

#[aoc_generator(day11)]
fn parse_input(data: &str) -> Plane {
    data.into()
}

#[aoc(day11, part1)]
fn part1(plane: &Plane) -> usize {
    plane.iterate_until_stable_part1().n_occupied_seats()
}

#[aoc(day11, part2)]
fn part2(plane: &Plane) -> usize {
    plane.iterate_until_stable_part2().n_occupied_seats()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &'static str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    const AFTER_ONE_ROUND: &'static str = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
    const AFTER_LAST_ROUND: &'static str = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

    fn input() -> Plane {
        parse_input(include_str!("../input/2020/day11.txt"))
    }
    #[test]
    fn test_one_iteration() {
        let plane = Plane::from(EXAMPLE).next_part1().unwrap();
        assert_eq!(Plane::from(AFTER_ONE_ROUND), plane)
    }
    #[test]
    fn test_all_iterations() {
        let last = Plane::from(EXAMPLE).iterate_until_stable_part1();
        assert_eq!(Plane::from(AFTER_LAST_ROUND), last)
    }
    #[test]
    fn test_part1_with_example() {
        assert_eq!(37, part1(&EXAMPLE.into()))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 2427)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 2199)
    }
}
