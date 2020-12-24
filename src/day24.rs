use aoc_runner_derive::{aoc, aoc_generator};
use direction::Coord;
use direction::Direction;
use rustc_hash::FxHashSet;

// 0 0 0 0 0 0 0 0 0 0 0 0 0 0
//  0 0 0 0 0 0 x 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0 0 0 0 0
#[derive(Clone)]
struct Tiles {
    // Note: could be seen as a VecDeque<VecDeque<bool>> with all the tiles in the region
    // containing black tiles.
    black_tiles: FxHashSet<Coord>,
}

impl From<&str> for Tiles {
    fn from(s: &str) -> Self {
        let mut black_tiles = FxHashSet::default();
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

impl Tiles {
    fn neighbours(coord: Coord) -> Vec<Coord> {
        [
            Direction::East.coord() * 2,
            Direction::West.coord() * 2,
            Direction::NorthEast.coord(),
            Direction::NorthWest.coord(),
            Direction::SouthEast.coord(),
            Direction::SouthWest.coord(),
        ]
        .iter()
        .map(move |dir| coord + dir)
        .collect()
    }
    fn n_adjacent_black_tiles(&self, coord: Coord) -> usize {
        Self::neighbours(coord)
            .iter()
            .filter(|coord| self.black_tiles.contains(coord))
            .count()
    }
    fn white_tiles(&self) -> impl Iterator<Item = Coord> + '_ {
        self.black_tiles
            .iter()
            .flat_map(move |coord| Self::neighbours(*coord))
            .filter(move |coord| !self.black_tiles.contains(coord))
    }
    fn game_of_life(&self) -> Self {
        let remain_black = self
            .black_tiles
            .iter()
            .filter(|coord| {
                let neighbours = self.n_adjacent_black_tiles(**coord);
                neighbours == 1 || neighbours == 2
            })
            .copied()
            .collect::<FxHashSet<_>>();
        let become_black = self
            .white_tiles()
            .filter(|coord| {
                let neighbours = self.n_adjacent_black_tiles(*coord);
                neighbours == 2
            })
            .collect();

        Self {
            black_tiles: remain_black.union(&become_black).copied().collect(),
        }
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

#[aoc(day24, part2)]
fn part2(floor: &Tiles) -> usize {
    let mut floor = floor.clone();
    for _ in 0..100 {
        floor = floor.game_of_life();
    }
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
    #[test]
    fn test_part2() {
        assert_eq!(4147, part2(&input()))
    }
}
