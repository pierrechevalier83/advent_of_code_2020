use aoc_runner_derive::aoc;
use direction::CardinalDirection;
use direction::Coord;

#[derive(Clone, Copy, Debug)]
enum RotationDirection {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Translate(CardinalDirection, usize),
    Rotate(RotationDirection, usize),
    Forward(usize),
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let (c, rest) = s.split_at(1);
        let n = rest.parse::<usize>().unwrap();
        match c {
            "N" => Self::Translate(CardinalDirection::North, n),
            "S" => Self::Translate(CardinalDirection::South, n),
            "E" => Self::Translate(CardinalDirection::East, n),
            "W" => Self::Translate(CardinalDirection::West, n),
            "L" => Self::Rotate(RotationDirection::Left, n),
            "R" => Self::Rotate(RotationDirection::Right, n),
            "F" => Self::Forward(n),
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum MoveRules {
    PartOne,
    PartTwo,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Ship {
    facing: CardinalDirection,
    position: Coord,
    waypoint: Coord,
    move_rules: MoveRules,
}

impl Ship {
    fn new(move_rules: MoveRules) -> Self {
        Self {
            facing: CardinalDirection::East,
            position: Coord::default(),
            waypoint: Coord::new(10, -1),
            move_rules,
        }
    }
    fn translate_coord(coord: Coord, direction: CardinalDirection, distance: usize) -> Coord {
        coord + direction.coord() * distance as i32
    }
    fn translate(&mut self, direction: CardinalDirection, distance: usize) {
        match self.move_rules {
            MoveRules::PartOne => {
                self.position = Self::translate_coord(self.position, direction, distance);
            }
            MoveRules::PartTwo => {
                self.waypoint = Self::translate_coord(self.waypoint, direction, distance);
            }
        }
    }
    fn rotate(&mut self, direction: RotationDirection, angle: usize) {
        let n_rotations = angle / 90;
        match self.move_rules {
            MoveRules::PartOne => {
                for _ in 0..n_rotations {
                    self.facing = match direction {
                        RotationDirection::Left => self.facing.left90(),
                        RotationDirection::Right => self.facing.right90(),
                    }
                }
            }
            MoveRules::PartTwo => {
                for _ in 0..n_rotations {
                    match direction {
                        RotationDirection::Left => {
                            self.waypoint = Coord::new(self.waypoint.y, -self.waypoint.x);
                        }
                        RotationDirection::Right => {
                            self.waypoint = Coord::new(-self.waypoint.y, self.waypoint.x);
                        }
                    };
                }
            }
        }
    }
    fn forward(&mut self, distance: usize) {
        match self.move_rules {
            MoveRules::PartOne => self.translate(self.facing, distance),
            MoveRules::PartTwo => {
                self.position = self.position + self.waypoint * distance as i32;
            }
        }
    }
    fn make_move(&mut self, m: Move) {
        match m {
            Move::Translate(direction, distance) => self.translate(direction, distance),
            Move::Rotate(direction, angle) => self.rotate(direction, angle),
            Move::Forward(distance) => self.forward(distance),
        }
    }
    fn manhattan_distance_to_start(&self) -> usize {
        self.position.manhattan_distance(Coord::default()) as usize
    }
}

#[aoc(day12, part1)]
fn part1(s: &str) -> usize {
    let mut ship = Ship::new(MoveRules::PartOne);
    for line in s.split_terminator('\n') {
        ship.make_move(Move::from(line));
    }
    ship.manhattan_distance_to_start()
}

#[aoc(day12, part2)]
fn part2(s: &str) -> usize {
    let mut ship = Ship::new(MoveRules::PartTwo);
    for line in s.split_terminator('\n') {
        ship.make_move(Move::from(line));
    }
    ship.manhattan_distance_to_start()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> &'static str {
        include_str!("../input/2020/day12.txt")
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1457)
    }
    #[test]
    fn test_example() {
        let moves = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(286, part2(moves))
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0)
    }
}
