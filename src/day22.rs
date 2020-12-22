use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

type Card = u8;

#[derive(Debug, Clone)]
struct Battle {
    // The players' decks of cards.
    // The top of the deck is at the back, the bottom is at the front
    decks: Vec<VecDeque<Card>>,
}

fn parse_card_deck(s: &str) -> VecDeque<u8> {
    s.split_terminator("\n")
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect()
}

impl From<&str> for Battle {
    fn from(s: &str) -> Self {
        Self {
            decks: s.split("\n\n").map(|s| parse_card_deck(s)).collect(),
        }
    }
}

impl Battle {
    fn play_round(&mut self) {
        let mut hands = self
            .decks
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect::<Vec<_>>();
        let winner: usize = hands
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| left.cmp(right))
            .map(|(index, _)| index)
            .unwrap();
        hands.sort_unstable_by(|left, right| right.cmp(left));
        self.decks[winner].extend(hands.iter())
    }
    fn play_game(&mut self) {
        while !self.decks.iter().any(|deck| deck.is_empty()) {
            self.play_round();
        }
    }
    fn count_score(&self) -> usize {
        self.decks
            .iter()
            .filter(|deck| !deck.is_empty())
            .map(|deck| {
                deck.iter()
                    .rev()
                    .enumerate()
                    .map(|(index, card)| *card as usize * (index + 1))
                    .sum()
            })
            .next()
            .unwrap()
    }
}

#[aoc_generator(day22)]
fn parse_input(s: &str) -> Battle {
    s.into()
}

#[aoc(day22, part1)]
fn part1(game: &Battle) -> usize {
    let mut game = game.clone();
    game.play_game();
    game.count_score()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Battle {
        parse_input(include_str!("../input/2020/day22.txt"))
    }
    const EXAMPLE: &'static str = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
    #[test]
    fn test_example() {
        assert_eq!(306, part1(&parse_input(EXAMPLE)))
    }
    #[test]
    fn test_part1() {
        assert_eq!(32856, part1(&input()))
    }
}
