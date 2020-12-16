use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Field {
    either: RangeInclusive<u32>,
    or: RangeInclusive<u32>,
}

impl Field {
    fn is_valid(&self, value: u32) -> bool {
        self.either.contains(&value) || self.or.contains(&value)
    }
}

fn parse_inclusive_range(s: &str) -> RangeInclusive<u32> {
    let (min, max) = s.splitn(2, '-').collect_tuple().unwrap();
    min.parse().unwrap()..=max.parse().unwrap()
}

impl From<&str> for Field {
    fn from(s: &str) -> Self {
        let (either, or) = s.splitn(2, " or ").collect_tuple().unwrap();
        Self {
            either: parse_inclusive_range(either),
            or: parse_inclusive_range(or),
        }
    }
}

#[derive(Debug, Clone)]
struct Ticket {
    field_values: Vec<u32>,
}

impl From<&str> for Ticket {
    fn from(s: &str) -> Self {
        Self {
            field_values: s.split(',').map(|num| num.parse().unwrap()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct TicketRules {
    fields: BTreeMap<String, Field>,
    our_ticket: Ticket,
    their_tickets: Vec<Ticket>,
}

impl From<&str> for TicketRules {
    fn from(s: &str) -> Self {
        let (fields_section, our_ticket_section, their_tickets_section) =
            s.splitn(3, "\n\n").collect_tuple().unwrap();
        let fields = fields_section
            .split_terminator('\n')
            .map(|line| {
                let (key, value) = line.splitn(2, ": ").collect_tuple().unwrap();
                (key.to_string(), value.into())
            })
            .collect();
        let our_ticket = our_ticket_section.splitn(2, "\n").nth(1).unwrap().into();
        let their_tickets = their_tickets_section
            .split_terminator("\n")
            .skip(1)
            .map(|ticket| ticket.into())
            .collect();
        Self {
            fields,
            our_ticket,
            their_tickets,
        }
    }
}

#[aoc_generator(day16)]
fn parse_input(data: &str) -> TicketRules {
    data.into()
}

#[aoc(day16, part1)]
fn part1(rules: &TicketRules) -> u32 {
    rules
        .their_tickets
        .iter()
        .flat_map(|ticket| {
            ticket
                .field_values
                .iter()
                .filter(|value| !rules.fields.values().any(|field| field.is_valid(**value)))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> TicketRules {
        parse_input(include_str!("../input/2020/day16.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 27850)
    }
}
