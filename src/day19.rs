use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::iter::repeat;

#[derive(Clone, Debug)]
enum Rule {
    A,
    B,
    Seq(Vec<usize>),
    EitherOr(Vec<usize>, Vec<usize>),
}

fn parse_ints(s: &str) -> Vec<usize> {
    s.split(' ').map(|x| x.parse().unwrap()).collect()
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        if s == "\"a\"" {
            Self::A
        } else if s == "\"b\"" {
            Self::B
        } else if s.contains("|") {
            let (either, or) = s.split(" | ").collect_tuple().unwrap();
            Self::EitherOr(parse_ints(either), parse_ints(or))
        } else {
            Self::Seq(parse_ints(s))
        }
    }
}

#[derive(Clone, Debug)]
struct Rules {
    // Sorted. The index in the Vec is the key to access the Rule
    rules: Vec<Rule>,
    // Feels wasteful to store a full byte of information with a char for something that has only
    // two states. An enum would probably be more expressive, but I'm feeling lazy. Let's just
    // agree true is "a" and false is "b" and not tell anyone. This will be our little secret ;)
    messages: Vec<Vec<bool>>,
}

impl From<&str> for Rules {
    fn from(s: &str) -> Self {
        let (rules_section, messages_section) = s.split("\n\n").collect_tuple().unwrap();
        let len = rules_section.split_terminator("\n").count();
        let mut rules = repeat(Rule::A).take(len).collect::<Vec<_>>();
        rules_section.split_terminator('\n').for_each(|line| {
            let (index, rule) = line.split(": ").collect_tuple().unwrap();
            rules[index.parse::<usize>().unwrap()] = Rule::from(rule);
        });
        let messages = messages_section
            .split_terminator("\n")
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c == 'a' {
                            true
                        } else if c == 'b' {
                            false
                        } else {
                            panic!("Expected only a or b chars in input for messages");
                        }
                    })
                    .collect()
            })
            .collect();
        Rules { rules, messages }
    }
}

impl Rules {
    // If matches the rule,
    // return Some(unmatched part of the message),
    // else None
    fn matches_rule<'a>(&self, message: &'a [bool], rule: &Rule) -> Option<&'a [bool]> {
        match rule {
            Rule::A => message
                .get(0)
                .and_then(|a| if *a { Some(&message[1..]) } else { None }),
            Rule::B => message
                .get(0)
                .and_then(|a| if !*a { Some(&message[1..]) } else { None }),
            Rule::Seq(rules) => {
                let mut part = message;
                for index in rules {
                    let next_part = self.matches_rule(part, &self.rules[*index]);
                    if let Some(next_part) = next_part {
                        part = next_part;
                    } else {
                        return None;
                    }
                }
                Some(part)
            }
            Rule::EitherOr(either, or) => self
                .matches_rule(message, &Rule::Seq(either.clone()))
                .or_else(|| self.matches_rule(message, &Rule::Seq(or.clone()))),
        }
    }
}

#[aoc_generator(day19)]
fn parse_input(s: &str) -> Rules {
    s.into()
}

#[aoc(day19, part1)]
fn part1(rules: &Rules) -> usize {
    rules
        .messages
        .iter()
        .filter(|msg| rules.matches_rule(msg, &rules.rules[0]) == Some(&[]))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Rules {
        parse_input(include_str!("../input/2020/day19.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 118)
    }
}
