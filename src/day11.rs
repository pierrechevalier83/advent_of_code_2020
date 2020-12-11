use aoc_runner_derive::{aoc, aoc_generator};
use either::Either;
use itertools::Itertools;
use std::fmt::{self, Debug, Formatter};
use std::iter::once;

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
    fn adjacent_indices(index: usize, len: usize) -> impl Iterator<Item = usize> {
        if index == 0 {
            Either::Left(once(index + 1).chain(once(index)))
        } else if index == len - 1 {
            Either::Left(once(index - 1).chain(once(index)))
        } else {
            Either::Right(once(index - 1).chain(once(index)).chain(once(index + 1)))
        }
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
    fn adjacent_seats(&self, pos: (usize, usize)) -> impl Iterator<Item = Seat> + '_ {
        Self::adjacent_indices(pos.0, self.n_rows())
            .cartesian_product(Self::adjacent_indices(pos.1, self.n_cols()).collect::<Vec<_>>())
            .filter(move |p| *p != pos)
            .map(move |pos| self.seat_at(pos))
    }
    fn updated_seat(&self, pos: (usize, usize)) -> Seat {
        if self.seat_at(pos) == Seat::Empty
            && !self.adjacent_seats(pos).any(|s| s == Seat::Occupied)
        {
            Seat::Occupied
        } else if self.seat_at(pos) == Seat::Occupied
            && self
                .adjacent_seats(pos)
                .filter(|s| *s == Seat::Occupied)
                .count()
                >= 4
        {
            Seat::Empty
        } else {
            self.seat_at(pos)
        }
    }
    fn n_occupied_seats(&self) -> usize {
        self.seats
            .iter()
            .flat_map(|row| row.iter())
            .filter(|s| **s == Seat::Occupied)
            .count()
    }
}

impl Iterator for Plane {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        let next = Self {
            seats: (0..self.n_rows())
                .map(|row| {
                    (0..self.n_cols())
                        .map(|col| self.updated_seat((row, col)))
                        .collect()
                })
                .collect(),
        };
        if next != *self {
            *self = next;
            Some(self.clone())
        } else {
            None
        }
    }
}

#[aoc_generator(day11)]
fn parse_input(data: &str) -> Plane {
    data.into()
}

#[aoc(day11, part1)]
fn part1(plane: &Plane) -> usize {
    plane.clone().last().unwrap().n_occupied_seats()
}

#[cfg(test)]
mod tests {
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
    use super::*;
    fn input() -> Plane {
        parse_input(include_str!("../input/2020/day11.txt"))
    }
    #[test]
    fn test_one_iteration() {
        let mut plane = Plane::from(EXAMPLE);
        plane.next();
        assert_eq!(Plane::from(AFTER_ONE_ROUND), plane)
    }
    #[test]
    fn test_all_iterations() {
        let last = Plane::from(EXAMPLE).last().unwrap();
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
}
