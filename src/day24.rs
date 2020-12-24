use aoc_runner_derive::{aoc, aoc_generator};
use direction::Coord;
use direction::Direction;
use std::collections::HashSet;

// 0 0 0 0 0 0 0 0 0 0 0 0 0 0
//  0 0 0 0 0 0 x 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0 0 0 0 0
struct Tiles {
    // Note: could be seen as a VecDeque<VecDeque<bool>> with all the tiles in the region
    // containing black tiles.
    black_tiles: HashSet<Coord>,
}

impl From<&str> for Tiles {
    fn from(s: &str) -> Self {
        let mut black_tiles = HashSet::new();
        s.split_terminator("\n").for_each(|line| {
            let mut coordinate = Coord::new(0, 0);
            let mut index = 0;
            while index < line.len() {
                coordinate += match &line[index..(index + 1)] {
                    "e" => Direction::East.coord() * 2,
                    "w" => Direction::West.coord() * 2,
                    _ => match &line[index..(index + 2)] {
                        "ne" => {
                            index += 1;
                            Direction::NorthEast.coord()
                        }
                        "nw" => {
                            index += 1;
                            Direction::NorthWest.coord()
                        }
                        "se" => {
                            index += 1;
                            Direction::SouthEast.coord()
                        }
                        "sw" => {
                            index += 1;
                            Direction::SouthWest.coord()
                        }
                        _ => panic!(format!(
                            "Expected only hexagonal directions, got: \"{}\"",
                            &s[index..(index + 2)]
                        )),
                    },
                };
                index += 1;
            }
            if !black_tiles.remove(&coordinate) {
                black_tiles.insert(coordinate);
            }
        });
        Self { black_tiles }
    }
}

#[aoc_generator(day24)]
fn parse_input(s: &str) -> Tiles {
    s.into()
}

#[aoc(day24, part1)]
fn part1(floor: &Tiles) -> usize {
    floor.black_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Tiles {
        parse_input(include_str!("../input/2020/day24.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(549, part1(&input()))
    }
}
