use aoc_runner_derive::aoc;
use std::iter::repeat;

fn puzzle_input(max_size: usize) -> Game {
    Game::from(vec![9, 6, 0, 10, 18, 2, 1], max_size)
}

#[derive(Debug)]
struct Game {
    last: u32,
    turn: u32,
    previously_seen: Vec<bool>,
    history: Vec<u32>,
}

impl Game {
    fn from(seq: Vec<u32>, max_size: usize) -> Self {
        let mut history = repeat(0).take(max_size).collect::<Vec<_>>();
        let mut previously_seen = repeat(false).take(max_size).collect::<Vec<_>>();
        seq.iter()
            .take(seq.len() - 1)
            .enumerate()
            .for_each(|(i, n)| {
                previously_seen[*n as usize] = true;
                history[*n as usize] = i as u32;
            });
        Self {
            last: *seq.last().unwrap(),
            turn: seq.len() as u32,
            previously_seen,
            history,
        }
    }
}

impl Iterator for Game {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let last = self.last as usize;
        let next = if self.previously_seen[last] {
            self.turn - 1 - self.history[last]
        } else {
            0
        };
        self.previously_seen[last] = true;
        self.history[last] = self.turn - 1;
        self.turn += 1;
        self.last = next;
        Some(next)
    }
}

#[aoc(day15, part1)]
fn part1(_: &str) -> u32 {
    let mut game = puzzle_input(2020);
    game.nth(2020 - 7 - 1).unwrap()
}
#[aoc(day15, part2)]
fn part2(_: &str) -> u32 {
    let mut game = puzzle_input(30_000_000);
    game.nth(30_000_000 - 7 - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        assert_eq!(Some(1), Game::from(vec![1, 3, 2], 2020).nth(2020 - 3 - 1));
        assert_eq!(Some(10), Game::from(vec![2, 1, 3], 2020).nth(2020 - 3 - 1));
        assert_eq!(Some(27), Game::from(vec![1, 2, 3], 2020).nth(2020 - 3 - 1));
        assert_eq!(Some(78), Game::from(vec![2, 3, 1], 2020).nth(2020 - 3 - 1));
        assert_eq!(Some(438), Game::from(vec![3, 2, 1], 2020).nth(2020 - 3 - 1));
        assert_eq!(
            Some(1836),
            Game::from(vec![3, 1, 2], 2020).nth(2020 - 3 - 1)
        );
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(""), 1238)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(""), 3745954)
    }
}
