use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

const MODULO: u64 = 20201227;

fn transform_subject_number(loop_size: u64, subject_number: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = value * subject_number % MODULO;
    }
    value
}

#[derive(Debug)]
enum Object {
    Card,
    Door,
}

fn get_smallest_loop_size_from_public_keys(public_keys: (u64, u64)) -> (Object, u64) {
    let mut value = 1;
    let subject_number = 7;
    (0..10_000_000)
        .find_map(move |loop_size| {
            if value == public_keys.0 {
                Some((Object::Card, loop_size))
            } else if value == public_keys.1 {
                Some((Object::Door, loop_size))
            } else {
                value = value * subject_number % MODULO;
                None
            }
        })
        .unwrap()
}

fn calculate_encryption_key(object: Object, loop_size: u64, public_keys: (u64, u64)) -> u64 {
    match object {
        Object::Card => transform_subject_number(loop_size, public_keys.1),
        Object::Door => transform_subject_number(loop_size, public_keys.0),
    }
}

fn reverse_engineer_encryption_key(public_keys: (u64, u64)) -> u64 {
    let (object, loop_size) = get_smallest_loop_size_from_public_keys(public_keys);
    calculate_encryption_key(object, loop_size, public_keys)
}

#[aoc_generator(day25)]
fn parse_input(s: &str) -> (u64, u64) {
    s.split_terminator("\n")
        .map(|line| line.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

#[aoc(day25, part1)]
fn part1(public_keys: &(u64, u64)) -> u64 {
    reverse_engineer_encryption_key(*public_keys)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> (u64, u64) {
        parse_input(include_str!("../input/2020/day25.txt"))
    }
    #[test]
    fn test_example() {
        let public_keys = (5764801, 17807724);
        assert_eq!(14897079, reverse_engineer_encryption_key(public_keys));
    }

    #[test]
    fn test_part1() {
        assert_eq!(17980581, part1(&input()))
    }
}
