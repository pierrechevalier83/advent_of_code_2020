use aoc_runner_derive::aoc;
use std::collections::{HashMap, VecDeque};
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    LeftParen,
    RightParen,
    Plus,
    Times,
    Num(u64),
}
// "(5 * 5 * 4 + 8) + 6 + ((5 + 5 + 2) + (5 + 6 + 2 * 4) * 2 * 9 * 4) + 7 + 4 * (6 + (6 * 6 + 7 * 3 + 5 * 3) * (6 + 6 * 9) * (3 * 7 + 8 + 2 * 5) + 4 + 7)"

fn tokenize(s: &str) -> Vec<Token> {
    s.chars()
        .filter(|c| *c != ' ')
        .map(|c| match c {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '+' => Token::Plus,
            '*' => Token::Times,
            c => Token::Num(char::to_digit(c, 10).unwrap() as u64),
        })
        .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct ParenthesisPair {
    opens_at: usize,
    closes_at: usize,
}

impl ParenthesisPair {
    fn new(opens_at: usize, closes_at: usize) -> Self {
        Self {
            opens_at,
            closes_at,
        }
    }
    fn content_offset(&self) -> usize {
        self.opens_at + 1
    }
    fn content(&self) -> Range<usize> {
        self.content_offset()..self.closes_at
    }
}

/// For each opening parenthesis,
fn all_matching_parentheses(tokens: &[Token]) -> VecDeque<ParenthesisPair> {
    let mut out = VecDeque::new();
    let mut stack = Vec::<usize>::new();
    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::RightParen => {
                stack.push(index);
            }
            Token::LeftParen => {
                out.push_back(ParenthesisPair::new(stack.pop().unwrap(), index));
            }
            _ => (),
        }
    }
    out
}

/// A term is either
/// * a scalar value, or
/// * a region between parenthesis (included) that was already evaluated
fn evaluate_term(
    tokens: &[Token],
    region: &Range<usize>,
    cache: &HashMap<usize, (Range<usize>, u64)>,
) -> Option<u64> {
    match tokens[region.start] {
        Token::Num(x) => {
            return Some(x);
        }
        _ => {
            if let Some((_region, value)) = cache.get(&(region.start + 1)) {
                assert_eq!(tokens[region.start], Token::RightParen);
                return Some(*value);
            }
        }
    }
    None
}

/// Returns the position of the next token after this term
fn term_end(
    tokens: &[Token],
    region: &Range<usize>,
    cache: &HashMap<usize, (Range<usize>, u64)>,
) -> usize {
    if tokens[region.start] == Token::RightParen {
        cache[&(region.start + 1)].0.end + 1
    } else {
        region.start + 1
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PrecedenceRule {
    None,
    PlusFirst,
}

/// Contract:
/// The input str
/// * is never empty
/// * any section between parenthesis was already evaluated
/// * any pair of parenthesis contained within it was previously evaluated
fn evaluate_expression(
    tokens: &[Token],
    region: &Range<usize>,
    cache: &HashMap<usize, (Range<usize>, u64)>,
    prec: PrecedenceRule,
) -> u64 {
    if term_end(tokens, region, cache) >= region.end {
        return evaluate_term(tokens, region, cache).unwrap();
    }

    let first_term_region = region.start..term_end(tokens, region, cache);
    let mut first_term = evaluate_term(tokens, &first_term_region, cache).unwrap();
    let mut op = tokens[first_term_region.end];
    let mut next_term_start = first_term_region.end + 1;
    while prec == PrecedenceRule::PlusFirst && op == Token::Plus {
        let next_term_region =
            next_term_start..term_end(tokens, &(next_term_start..region.end), cache);
        if next_term_region.end < region.end {
            op = tokens[next_term_region.end];
            next_term_start = next_term_region.end + 1;
            first_term += evaluate_term(tokens, &next_term_region, cache).unwrap();
        } else {
            break;
        }
    }
    let remaining_region = next_term_start..region.end;
    let remaining_term = evaluate_expression(tokens, &remaining_region, cache, prec);

    let res = match op {
        Token::Plus => first_term + remaining_term,
        Token::Times => first_term * remaining_term,
        _ => panic!("Not an operator: {:?}", op),
    };
    res
}

fn parse_expression(tokens: &[Token], prec: PrecedenceRule) -> u64 {
    let parens = all_matching_parentheses(tokens);
    // TODO: perf: once it works, Vec<u64> where the index represents the key is probably more
    //             efficient
    let mut cache = HashMap::new();
    for p in parens {
        let offset = p.content_offset();
        cache.insert(
            offset,
            (
                p.content(),
                evaluate_expression(tokens, &p.content(), &cache, prec),
            ),
        );
    }
    evaluate_expression(tokens, &(0..tokens.len()), &cache, prec)
}

#[aoc(day18, part1)]
fn part1(s: &str) -> u64 {
    s.split_terminator('\n')
        .map(|line| {
            let mut tokens = tokenize(line);
            tokens.reverse();
            parse_expression(&tokens, PrecedenceRule::None)
        })
        .sum()
}

#[aoc(day18, part2)]
fn part2(s: &str) -> u64 {
    s.split_terminator('\n')
        .map(|line| {
            let mut tokens = tokenize(line);
            tokens.reverse();
            parse_expression(&tokens, PrecedenceRule::PlusFirst)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> &'static str {
        include_str!("../input/2020/day18.txt")
    }
    #[test]
    fn test_precedence() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(71, part1(input));
    }
    #[test]
    fn test_parens() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(51, part1(input));
    }
    #[test]
    fn test_plus_precedence() {
        let input = "2 * 3 + (4 * 5)";
        assert_eq!(46, part2(input));
        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(1445, part2(input));
    }
    #[test]
    fn test_part1() {
        assert_eq!(6923486965641, part1(&input()))
    }
    #[test]
    fn test_part2() {
        assert_eq!(70722650566361, part2(&input()))
    }
}
