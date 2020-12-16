use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Field {
    either: RangeInclusive<u64>,
    or: RangeInclusive<u64>,
}

impl Field {
    fn is_valid(&self, value: u64) -> bool {
        self.either.contains(&value) || self.or.contains(&value)
    }
}

fn parse_inclusive_range(s: &str) -> RangeInclusive<u64> {
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
    field_values: Vec<u64>,
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

impl<'a> TicketRules {
    fn never_valid_values(&'a self, ticket: &'a Ticket) -> impl Iterator<Item = u64> + 'a {
        ticket
            .field_values
            .iter()
            .copied()
            .filter(move |value| !self.fields.values().any(|field| field.is_valid(*value)))
    }
    fn is_ticket_valid(&self, ticket: &Ticket) -> bool {
        self.never_valid_values(ticket).next().is_none()
    }
    fn n_fields(&self) -> usize {
        self.our_ticket.field_values.len()
    }
    fn col_may_apply_to_this_field(&self, col: usize, field: Field) -> bool {
        !self
            .their_tickets
            .iter()
            .filter(|tkt| self.is_ticket_valid(tkt))
            .map(|tkt| tkt.field_values[col])
            .any(|value| !field.is_valid(value))
    }
    fn exclusion_matrix(&self) -> Vec<Vec<bool>> {
        (0..self.n_fields())
            .map(|col| {
                self.fields
                    .values()
                    .map(|field| self.col_may_apply_to_this_field(col, field.clone()))
                    .collect()
            })
            .collect()
    }
    fn fields_mapping(&self) -> Vec<String> {
        let mut fields_mapping = (0..self.fields.len())
            .map(|_| None)
            .collect::<Vec<Option<String>>>();
        let exclusion_matrix = self.exclusion_matrix();
        let mut num_mapped = 0;
        while num_mapped < self.fields.len() {
            exclusion_matrix
                .iter()
                .enumerate()
                .for_each(|(col_index, col)| {
                    let mut possible_new_mappings = col
                        .iter()
                        .enumerate()
                        .filter(|(field_index, _)| {
                            let field_key = self.fields.keys().nth(*field_index).unwrap();
                            !fields_mapping.contains(&Some(field_key.clone()))
                        })
                        .filter(|(_, x)| **x);
                    if let Some((field_index, _)) = possible_new_mappings.next() {
                        if possible_new_mappings.next().is_none() {
                            fields_mapping[col_index] =
                                Some(self.fields.keys().nth(field_index).unwrap().clone());
                            num_mapped += 1;
                        }
                    }
                });
        }
        fields_mapping.iter().flatten().cloned().collect()
    }
}

#[aoc(day16, part1)]
fn part1(data: &str) -> u64 {
    let rules = TicketRules::from(data);
    rules
        .their_tickets
        .iter()
        .flat_map(|ticket| rules.never_valid_values(ticket))
        .sum()
}

#[aoc(day16, part2)]
fn part2(data: &str) -> u64 {
    let rules = TicketRules::from(data);
    let fields_mapping = rules.fields_mapping();
    fields_mapping
        .iter()
        .enumerate()
        .filter_map(|(index, key)| {
            if key.clone().starts_with("departure") {
                let value = rules.our_ticket.field_values[index];
                Some(value)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> &'static str {
        include_str!("../input/2020/day16.txt")
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 27850)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 491924517533)
    }
}
