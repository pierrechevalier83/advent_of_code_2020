use aoc_runner_derive::aoc;
use std::iter::repeat;

#[derive(Clone, Debug)]
struct Cups {
    // For each value (index, zero based), the next value (zero based)
    cups: Vec<usize>,
    current: usize,
}

impl From<&str> for Cups {
    fn from(s: &str) -> Cups {
        Self::new(s, None)
    }
}

impl Cups {
    fn new(s: &str, pad_to: Option<usize>) -> Self {
        let n = s.chars().filter(|c| *c != '\n').count();
        let input = s
            .chars()
            .filter_map(|c| c.to_digit(10).map(|digit| (digit - 1) as usize));

        let current = input.clone().next().unwrap();
        let last = pad_to
            .map(|pad_to| pad_to - 1)
            .unwrap_or(input.clone().last().unwrap());
        let mut cups = repeat(0).take(pad_to.unwrap_or(n)).collect::<Vec<_>>();
        let mappings = input.chain(n..pad_to.unwrap_or(n)).collect::<Vec<_>>();

        for two_cups in mappings.windows(2) {
            cups[two_cups[0]] = two_cups[1];
        }
        cups[last] = current;
        Self { cups, current }
    }
    fn wrap_decrement(&self, x: usize) -> usize {
        if x == 0 {
            (self.cups.len() - 1) as usize
        } else {
            x - 1
        }
    }
    fn next_move(&mut self) {
        // The crab picks up the three cups that are immediately clockwise of
        // the current cup. They are removed from the circle; cup spacing is
        // adjusted as necessary to maintain the circle.
        let next = self.cups[self.current];
        let second = self.cups[next];
        let third = self.cups[second];
        let fourth = self.cups[third];
        let picked_up = [next, second, third];
        // The crab selects a destination cup: the cup with a label equal to
        // the current cup's label minus one. If this would select one of the
        // cups that was just picked up, the crab will keep subtracting one
        // until it finds a cup that wasn't just picked up. If at any point in
        // this process the value goes below the lowest value on any cup's
        // label, it wraps around to the highest value on any cup's label
        // instead.
        let mut destination = self.wrap_decrement(self.current);
        while picked_up.iter().find(|cup| **cup == destination).is_some() {
            destination = self.wrap_decrement(destination);
        }
        let after_destination = self.cups[destination];
        // The crab places the cups it just picked up so that they are
        // immediately clockwise of the destination cup. They keep the same
        // order as when they were picked up.
        self.cups[destination] = next;
        self.cups[third] = after_destination;
        self.cups[self.current] = fourth;
        self.current = fourth;
    }
    fn nth_move(&mut self, n: usize) {
        for _ in 0..n {
            self.next_move();
        }
    }
    fn order_string(&self) -> String {
        let mut order = vec![];
        let mut next = self.cups[0];
        while next != 0 {
            order.push(next);
            next = self.cups[next];
        }
        order
            .iter()
            .map(|number| format!("{}", number + 1))
            .collect()
    }
    fn next_two_cups(&self) -> [usize; 2] {
        let next = self.cups[0];
        let second = self.cups[next];
        [next + 1, second + 1]
    }
}

#[aoc(day23, part1)]
fn part1(s: &str) -> String {
    let mut cups = Cups::from(s);
    cups.nth_move(100);
    cups.order_string()
}
#[aoc(day23, part2)]
fn part2(s: &str) -> usize {
    let mut cups = Cups::new(s, Some(1_000_000));
    cups.nth_move(10_000_000);
    cups.next_two_cups().iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> &'static str {
        include_str!("../input/2020/day23.txt")
    }
    const EXAMPLE: &'static str = "389125467";
    #[test]
    fn test_example_10_moves() {
        let mut cups = Cups::from(EXAMPLE);
        cups.nth_move(10);
        assert_eq!("92658374", cups.order_string())
    }
    #[test]
    fn test_example_100_moves() {
        let mut cups = Cups::from(EXAMPLE);
        cups.nth_move(100);
        assert_eq!("67384529", cups.order_string());
    }
    #[test]
    fn test_part1() {
        assert_eq!("25468379", part1(&input()))
    }
    #[test]
    fn test_example_part2() {
        assert_eq!(149245887792, part2(EXAMPLE));
    }
}
