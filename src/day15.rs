use aoc_runner_derive::aoc;
use std::collections::HashMap;

fn puzzle_input() -> Game {
    Game::from(vec![9, 6, 0, 10, 18, 2, 1])
}

#[derive(Debug)]
struct Game {
    len: usize,
    last: usize,
    turn: usize,
    previously_seen: HashMap<usize, usize>,
}

impl From<Vec<usize>> for Game {
    fn from(seq: Vec<usize>) -> Self {
        Self {
            len: seq.len(),
            last: *seq.last().unwrap(),
            turn: seq.len(),
            previously_seen: seq
                .iter()
                .take(seq.len() - 1)
                .copied()
                .enumerate()
                .map(|(i, n)| (n, i))
                .collect(),
        }
    }
}

impl Iterator for Game {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let next = self
            .previously_seen
            .get(&self.last)
            .map(|previously_seen_at| self.turn - 1 - previously_seen_at)
            .unwrap_or(0);
        self.previously_seen.insert(self.last, self.turn - 1);
        self.turn += 1;
        self.last = next;
        Some(next)
    }
}

#[aoc(day15, part1)]
fn part1(_: &str) -> usize {
    let mut game = puzzle_input();
    game.nth(2020 - game.len - 1).unwrap()
}
#[aoc(day15, part2)]
fn part2(_: &str) -> usize {
    let mut game = puzzle_input();
    game.nth(30000000 - game.len - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        assert_eq!(Some(1), Game::from(vec![1, 3, 2]).nth(2020 - 3 - 1));
        assert_eq!(Some(10), Game::from(vec![2, 1, 3]).nth(2020 - 3 - 1));
        assert_eq!(Some(27), Game::from(vec![1, 2, 3]).nth(2020 - 3 - 1));
        assert_eq!(Some(78), Game::from(vec![2, 3, 1]).nth(2020 - 3 - 1));
        assert_eq!(Some(438), Game::from(vec![3, 2, 1]).nth(2020 - 3 - 1));
        assert_eq!(Some(1836), Game::from(vec![3, 1, 2]).nth(2020 - 3 - 1));
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
