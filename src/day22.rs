use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::{FxHashSet, FxHasher};
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::iter::repeat;

type Card = u8;

#[derive(Debug)]
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
    fn play_game(&mut self) -> usize {
        while !self.decks.iter().any(|deck| deck.is_empty()) {
            self.play_round();
        }
        self.decks.iter().position(|deck| !deck.is_empty()).unwrap()
    }
}

fn count_score(deck: &VecDeque<Card>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(index, card)| *card as usize * (index + 1))
        .sum()
}

#[derive(Debug)]
struct RecursiveCombat {
    previous_rounds: Vec<FxHashSet<u64>>,
    decks: Vec<VecDeque<Card>>,
}

impl RecursiveCombat {
    fn new(decks: Vec<VecDeque<Card>>) -> Self {
        let previous_rounds = repeat(FxHashSet::default()).take(decks.len()).collect();
        Self {
            previous_rounds,
            decks,
        }
    }
    fn play_round(&mut self) -> bool {
        // Before either player deals a card, if there was a previous round in
        // this game that had exactly the same cards in the same order in the
        // same players' decks, the game instantly ends in a win for player 1.
        // Previous rounds from other games are not considered. (This prevents
        // infinite games of Recursive Combat, which everyone agrees is a bad
        // idea.)
        let deck_hashes = self
            .decks
            .iter()
            .map(|deck| {
                let mut hasher = FxHasher::default();
                deck.hash(&mut hasher);
                hasher.finish()
            })
            .collect::<Vec<_>>();
        if self
            .previous_rounds
            .iter()
            .zip(deck_hashes.iter())
            .any(|(previously_played, deck_hash)| previously_played.contains(deck_hash))
        {
            return false;
        }
        for (index, deck_hash) in deck_hashes.iter().enumerate() {
            self.previous_rounds[index].insert(*deck_hash);
        }
        let mut hands = self
            .decks
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect::<Vec<_>>();

        // If both players have at least as many cards remaining in their deck
        // as the value of the card they just drew, the winner of the round is
        // determined by playing a new game of Recursive Combat (see below).
        let winner = if hands
            .iter()
            .zip(self.decks.iter())
            .all(|(hand, deck)| deck.len() >= *hand as usize)
        {
            let mut sub_game = RecursiveCombat::new(
                self.decks
                    .iter()
                    .zip(hands.iter())
                    .map(|(deck, hand)| deck.iter().take(*hand as usize).copied().collect())
                    .collect(),
            );
            sub_game.play_game()
        } else {
            hands
                .iter()
                .enumerate()
                .max_by(|(_, left), (_, right)| left.cmp(right))
                .map(|(index, _)| index)
                .unwrap()
        };
        // So ends the generality of our solution to more than 2 players
        hands = vec![hands[winner], hands[1 - winner]];
        self.decks[winner].extend(hands.iter());
        true
    }
    fn play_game(&mut self) -> usize {
        while !self.decks.iter().any(|deck| deck.is_empty()) {
            if !self.play_round() {
                return 0;
            }
        }
        let winner = self.decks.iter().position(|deck| !deck.is_empty()).unwrap();
        winner
    }
}

#[aoc_generator(day22)]
fn parse_input(s: &str) -> Vec<VecDeque<Card>> {
    s.split("\n\n").map(|s| parse_card_deck(s)).collect()
}

#[aoc(day22, part1)]
fn part1(decks: &[VecDeque<Card>]) -> usize {
    let mut game = Battle {
        decks: decks.to_vec(),
    };
    let winner = game.play_game();
    count_score(&game.decks[winner])
}

#[aoc(day22, part2)]
fn part2(decks: &[VecDeque<Card>]) -> usize {
    let mut game = RecursiveCombat::new(decks.to_vec());
    let winner = game.play_game();
    count_score(&game.decks[winner])
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Vec<VecDeque<Card>> {
        parse_input(include_str!("../input/2020/day22.txt"))
    }
    const EXAMPLE: &'static str = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
    const INFINITE_RECURSION_EXAMPLE: &'static str = "Player 1:\n43\n19\n\nPlayer 2:\n2\n29\n14";
    #[test]
    fn test_example_part1() {
        assert_eq!(306, part1(&parse_input(EXAMPLE)))
    }
    #[test]
    fn test_example_part2() {
        assert_eq!(291, part2(&parse_input(EXAMPLE)))
    }
    #[test]
    fn test_example_part2_infinite_recursion() {
        assert_eq!(105, part2(&parse_input(INFINITE_RECURSION_EXAMPLE)))
    }
    #[test]
    fn test_part1() {
        assert_eq!(32856, part1(&input()))
    }
    #[test]
    fn test_part2() {
        assert_eq!(33805, part2(&input()))
    }
}
