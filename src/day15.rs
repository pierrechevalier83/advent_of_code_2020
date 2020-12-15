use aoc_runner_derive::aoc;

fn puzzle_input() -> Game {
    Game::from(vec![9, 6, 0, 10, 18, 2, 1])
}

#[derive(Debug)]
struct Game {
    numbers: Vec<usize>,
}

impl From<Vec<usize>> for Game {
    fn from(seq: Vec<usize>) -> Self {
        Self { numbers: seq }
    }
}

impl Iterator for Game {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let turn = self.numbers.len();
        let last = self.numbers.last().copied().unwrap();
        let last_seen_at = self.numbers.iter().rev().position(|x| *x == last);
        let previously_seen_at = self
            .numbers
            .iter()
            .rev()
            // skip the one we've already seen
            .skip(1 + last_seen_at.unwrap_or(0))
            .position(|x| *x == last)
            // count the one we skipped
            .map(|x| x + 1);
        let next = previously_seen_at.unwrap_or(last_seen_at.unwrap_or(turn));
        self.numbers.push(next);
        println!("{}", next);
        Some(next)
    }
}

#[aoc(day15, part1)]
fn part1(_: &str) -> usize {
    let mut game = puzzle_input();
    game.nth(2020 - game.numbers.len() - 1).unwrap()
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
}
