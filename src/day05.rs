use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashSet, str::FromStr};

struct Ticket {
    row: u8,
    col: u8,
}

impl FromStr for Ticket {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binary_row = s[0..7]
            .chars()
            .map(|c| match c {
                'F' => '0',
                'B' => '1',
                _ => panic!("Expected only F or B for the first 7 characters"),
            })
            .collect::<String>();
        let binary_col = s[7..]
            .chars()
            .map(|c| match c {
                'L' => '0',
                'R' => '1',
                _ => panic!("Expected only L or R for the last 3 characters"),
            })
            .collect::<String>();
        Ok(Self {
            row: u8::from_str_radix(&binary_row, 2)?,
            col: u8::from_str_radix(&binary_col, 2)?,
        })
    }
}

impl Ticket {
    fn uid(&self) -> u16 {
        8 * self.row as u16 + self.col as u16
    }
}

#[aoc_generator(day5)]
fn parse_input(data: &str) -> Vec<Ticket> {
    data.split_terminator("\n")
        .map(|s| Ticket::from_str(s).unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn part1(tickets: &[Ticket]) -> u16 {
    tickets.iter().map(|ticket| ticket.uid()).max().unwrap()
}

#[aoc(day5, part2)]
fn part2(tickets: &[Ticket]) -> u16 {
    let ticket_ids = tickets
        .iter()
        .map(|ticket| ticket.uid())
        .collect::<HashSet<_>>();
    let max_ticket_id = Ticket { row: 127, col: 7 }.uid();
    (1..(max_ticket_id - 1))
        .find(|ticket| {
            !ticket_ids.contains(ticket)
                && ticket_ids.contains(&(*ticket - 1))
                && ticket_ids.contains(&(*ticket + 1))
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_ticket() {
        assert_eq!(357, Ticket::from_str("FBFBBFFRLR").unwrap().uid());
        assert_eq!(567, Ticket::from_str("BFFFBBFRRR").unwrap().uid());
        assert_eq!(119, Ticket::from_str("FFFBBBFRRR").unwrap().uid());
        assert_eq!(820, Ticket::from_str("BBFFBBFRLL").unwrap().uid());
    }
    fn input() -> Vec<Ticket> {
        parse_input(include_str!("../input/2020/day5.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 813)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 612)
    }
}
