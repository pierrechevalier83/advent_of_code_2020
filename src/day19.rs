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

#[aoc(day19, part2)]
fn part2(rules: &Rules) -> usize {
    // Before:
    // 0: 8 11
    // 8: 42
    // 11: 42 31
    // Now:
    // 0: 8 11
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    //
    // Rule 8 now means: any number of instances of rule 42 greater than one
    // Rule 11 now means: EitherOr([42 31], [42 11 31]), which is to say: either 42 31 or a
    // certain number of 42s followed by the same number of 31s.
    //
    // So rule 0 becomes:
    // N 42s and M 31s where N >= 2 and M < N
    //
    // From the original case,
    // We can keep adding 42s in the front.
    // If a message matches, we can count it and stop looking at it in the future
    // If a message doesn't match the N 42s, it can be ruled out and not counted
    // We will eventually have counted or ruled out all messages
    let mut n_matching = 0;
    let mut n_unknown = rules.messages.len();
    let mut unknown_messages = repeat(true).take(rules.messages.len()).collect::<Vec<_>>();
    for n_42s in 2..100 {
        for n_31s in 1..n_42s {
            let previously_unknown = unknown_messages.clone();
            for (index, message) in rules
                .messages
                .iter()
                .enumerate()
                .filter(|(index, _)| previously_unknown[*index])
            {
                let first_rule = Rule::Seq(repeat(42).take(n_42s).collect());
                if let Some(second_part) = rules.matches_rule(message, &first_rule) {
                    let second_rule = Rule::Seq(repeat(31).take(n_31s).collect());
                    if let Some(&[]) = rules.matches_rule(second_part, &second_rule) {
                        n_matching += 1;
                        n_unknown -= 1;
                        unknown_messages[index] = false;
                    }
                } else {
                    n_unknown -= 1;
                    unknown_messages[index] = false;
                }
                if n_unknown == 0 {
                    return n_matching;
                }
            }
        }
    }
    panic!("This should be unreachable as we should have ran out of unknowns earlier");
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
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 246)
    }
}
