use aoc_runner_derive::{aoc, aoc_generator};
use direction::CardinalDirection;
use direction::Coord;

enum ShipMove {
    Translate(CardinalDirection, usize),
    RotateLeft(usize),
    RotateRight(usize),
    Forward(usize),
}

impl From<&str> for ShipMove {
    fn from(s: &str) -> Self {
        let (c, rest) = s.split_at(1);
        let n = rest.parse::<usize>().unwrap();
        match c {
            "N" => Self::Translate(CardinalDirection::North, n),
            "S" => Self::Translate(CardinalDirection::South, n),
            "E" => Self::Translate(CardinalDirection::East, n),
            "W" => Self::Translate(CardinalDirection::West, n),
            "L" => Self::RotateLeft(n),
            "R" => Self::RotateRight(n),
            "F" => Self::Forward(n),
            _ => panic!("Invalid input"),
        }
    }
}

struct Ship {
    facing: CardinalDirection,
    position: Coord,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            facing: CardinalDirection::East,
            position: Coord::default(),
        }
    }
}

impl Ship {
    fn translate(&self, direction: CardinalDirection, distance: usize) -> Self {
        Self {
            facing: self.facing,
            position: self.position + direction.coord() * distance as i32,
        }
    }
    fn rotate_left(&self, angle: usize) -> Self {
        let n_rotations = angle / 90;
        let mut facing = self.facing;
        for _ in 0..n_rotations {
            facing = facing.left90();
        }
        Self {
            facing,
            position: self.position,
        }
    }
    fn rotate_right(&self, angle: usize) -> Self {
        let n_rotations = angle / 90;
        let mut facing = self.facing;
        for _ in 0..n_rotations {
            facing = facing.right90();
        }
        Self {
            facing,
            position: self.position,
        }
    }
    fn make_move(&self, m: ShipMove) -> Self {
        match m {
            ShipMove::Translate(direction, distance) => self.translate(direction, distance),
            ShipMove::RotateRight(angle) => self.rotate_right(angle),
            ShipMove::RotateLeft(angle) => self.rotate_left(angle),
            ShipMove::Forward(distance) => self.translate(self.facing, distance),
        }
    }
}

#[aoc_generator(day12)]
fn parse_input(s: &str) -> Ship {
    let mut ship = Ship::default();
    for line in s.split_terminator('\n') {
        ship = ship.make_move(ShipMove::from(line));
    }
    ship
}

#[aoc(day12, part1)]
fn part1(ship: &Ship) -> usize {
    ship.position.manhattan_distance(Coord::new(0, 0)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Ship {
        parse_input(include_str!("../input/2020/day12.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 1457)
    }
}
