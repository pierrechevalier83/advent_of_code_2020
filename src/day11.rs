use aoc_runner_derive::{aoc};
use itertools::Itertools;
use std::fmt::{self, Debug, Formatter};
use std::iter::repeat;

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
    visited: Vec<Vec<bool>>,
    adjacent_seats: Vec<Vec<Vec<(usize, usize)>>>,
    visible_seats: Vec<Vec<Vec<(usize, usize)>>>,
    n_people: usize,
    n_cols: usize,
    n_rows: usize,
    n_visited_by_rows: Vec<usize>,
    n_visited_by_cols: Vec<usize>,
}

impl Debug for Plane {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        for row in 0..self.n_rows {
            for col in 0..self.n_cols {
                if !self.visited((row, col)) {
                    write!(f, "{:?}", Seat::Empty)?;
                } else {
                    write!(f, "🟩")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
impl From<&str> for Plane {
    fn from(s: &str) -> Self {
        let n_cols = s.chars().position(|c| c == '\n').unwrap();
        let n_rows = s.chars().filter(|c| *c != '\n').count() / n_cols;
        let mut n_visited_by_rows = repeat(0).take(n_rows).collect::<Vec<_>>();
        let mut n_visited_by_cols = repeat(0).take(n_cols).collect::<Vec<_>>();
        let visited = s
            .split_terminator('\n')
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .map(move |c| Seat::from(c))
                    .enumerate()
                    .map(|(col, seat)| {
                        if seat == Seat::Floor {
                            n_visited_by_rows[row] += 1;
                            n_visited_by_cols[col] += 1;
                            true
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            visited,
            n_people: 0,
            n_cols,
            n_rows,
            n_visited_by_rows,
            n_visited_by_cols,
            adjacent_seats: vec![],
            visible_seats: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Neighbourhood {
    Adjacency,
    Visibility,
}

impl<'a> Plane {
    fn visited(&self, (row, col): (usize, usize)) -> bool {
        self.n_visited_by_rows[row] == self.n_cols
            || self.n_visited_by_cols[col] == self.n_rows
            || self.visited[row][col]
    }
    fn mark_visited(&mut self, (row, col): (usize, usize)) {
        if !self.visited((row, col)) {
            self.visited[row][col] = true;
            self.n_visited_by_rows[row] += 1;
            self.n_visited_by_cols[col] += 1;
        }
    }
    fn all_directions() -> Vec<(isize, isize)> {
        let flat = [-1, 0, 1];
        flat.iter()
            .copied()
            .cartesian_product(flat.iter().copied())
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
        Self::adjacent_index(pos.0, dir.0, self.n_rows).and_then(|row| {
            Self::adjacent_index(pos.1, dir.1, self.n_cols).and_then(|col| {
                if self.visited((row, col)) {
                    None
                } else {
                    Some((row, col))
                }
            })
        })
    }
    fn adjacent_seats(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        Self::all_directions()
            .iter()
            .filter_map(|dir| self.adjacent_position(pos, *dir))
            .collect()
    }
    fn precompute_adjacent_seats(&mut self) {
        let neighbours = (0..self.n_rows)
            .map(|row| {
                (0..self.n_cols)
                    .map(|col| self.adjacent_seats((row, col)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        self.adjacent_seats = neighbours;
    }
    fn visible_position(&self, pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
        if dir == (0, 0) {
            if self.visited(pos) {
                return None;
            } else {
                return Some(pos);
            }
        }
        let mut pos = pos;
        let mut ret = None;
        while ret.is_none() {
            let row = Self::adjacent_index(pos.0, dir.0, self.n_rows);
            let col = Self::adjacent_index(pos.1, dir.1, self.n_cols);
            if row.is_none() || col.is_none() {
                return None;
            }
            pos = (row.unwrap(), col.unwrap());
            ret = if self.visited(pos) { None } else { Some(pos) }
        }
        return ret;
    }
    fn visible_seats(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        Self::all_directions()
            .iter()
            .filter_map(|dir| self.visible_position(pos, *dir))
            .collect()
    }
    fn precompute_visible_seats(&mut self) {
        let neighbours = (0..self.n_rows)
            .map(|row| {
                (0..self.n_cols)
                    .map(|col| self.visible_seats((row, col)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        self.visible_seats = neighbours;
    }
    fn next(&mut self, neighbourhood: Neighbourhood, num_tolerated: usize) -> bool {
        let mut num_seen = 0;
        let visited_neighbours = (0..self.n_rows)
            .filter(|row| self.n_visited_by_rows[*row] < self.n_cols)
            .flat_map(|row| {
                (0..self.n_cols)
                    .filter(|col| self.n_visited_by_cols[*col] < self.n_cols)
                    .filter(|col| !self.visited[row][*col])
                    .flat_map(|col| {
                        let neighbours = match neighbourhood {
                            Neighbourhood::Adjacency => &self.adjacent_seats,
                            Neighbourhood::Visibility => &self.visible_seats,
                        };
                        if neighbours[row][col]
                            .iter()
                            .filter(|n| !self.visited[n.0][n.1])
                            .count()
                            < num_tolerated + 1
                        {
                            num_seen += 1;

                            neighbours[row][col].clone()
                        } else {
                            vec![]
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        visited_neighbours
            .iter()
            .for_each(|n| self.mark_visited(*n));
        self.n_people += num_seen;

        if self.n_visited_by_rows.iter().sum::<usize>() == self.n_rows * self.n_cols {
            false
        } else {
            true
        }
    }
    fn iterate_until_stable(&mut self, neighbourhood: Neighbourhood, num_tolerated: usize) {
        match neighbourhood {
            Neighbourhood::Adjacency => self.precompute_adjacent_seats(),
            Neighbourhood::Visibility => self.precompute_visible_seats(),
        }
        while self.next(neighbourhood, num_tolerated) {
            // keep looping
        }
    }
}

#[aoc(day11, part1)]
fn part1(s: &str) -> usize {
    let mut plane = Plane::from(s);
    plane.iterate_until_stable(Neighbourhood::Adjacency, 4);
    plane.n_people
}

#[aoc(day11, part2)]
fn part2(s: &str) -> usize {
    let mut plane = Plane::from(s);
    plane.iterate_until_stable(Neighbourhood::Visibility, 5);
    plane.n_people
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
    fn input() -> &'static str {
        include_str!("../input/2020/day11.txt")
    }
    #[test]
    fn test_part1_with_example() {
        assert_eq!(37, part1(&EXAMPLE))
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
