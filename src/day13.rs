use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::iter::once;

#[derive(Debug)]
struct Input {
    earliest: usize,
    buses: Vec<Option<usize>>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let (first_line, second_line) = s.split_once("\n").unwrap();
        Input {
            earliest: first_line.parse().unwrap(),
            buses: second_line
                .split_terminator(",")
                .map(|s| s.parse().ok())
                .collect(),
        }
    }
}

impl Input {
    fn target_time(index: usize, bus: usize) -> usize {
        if index == 0 {
            0
        } else {
            bus - index % bus
        }
    }
    fn time_to_wait(&self, bus: usize) -> usize {
        Self::target_time(self.earliest, bus)
    }
    fn bus_with_least_wait(&self) -> (usize, usize) {
        self.buses
            .iter()
            .filter_map(|bus| {
                bus.map(|bus| {
                    let wait = self.time_to_wait(bus);
                    (bus, wait)
                })
            })
            .min_by(|(_, left_wait), (_, right_wait)| left_wait.cmp(right_wait))
            .unwrap()
    }
    fn product_other_buses(&self, bus: usize) -> usize {
        self.buses
            .iter()
            .filter_map(|b| b.and_then(|b| if b == bus { None } else { Some(b) }))
            .product()
    }
    fn complement(&self, bus: usize, product: usize, target: usize) -> usize {
        (0..bus)
            .find(|complement| complement * product % bus == target)
            .unwrap()
    }
}

#[aoc_generator(day13)]
fn parse_input(s: &str) -> Input {
    Input::from(s)
}

#[aoc(day13, part1)]
fn part1(timetable: &Input) -> usize {
    let (bus, wait) = timetable.bus_with_least_wait();
    wait * bus
}

#[aoc(day13, part2)]
fn part2(timetable: &Input) -> usize {
    timetable
        .buses
        .iter()
        .enumerate()
        .filter_map(|(index, bus)| {
            bus.map(|bus| {
                let target = Input::target_time(index, bus);
                let product = timetable.product_other_buses(bus);
                let pos_complement = timetable.complement(bus, product, target) as isize;
                let neg_complement = pos_complement - bus as isize;
                once(product as isize * neg_complement)
                    .chain(once(product as isize * pos_complement))
            })
        })
        .multi_cartesian_product()
        .map(|terms| terms.iter().sum())
        .filter(|term: &isize| *term > 0)
        .min()
        .unwrap() as usize
}
#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Input {
        parse_input(include_str!("../input/2020/day13.txt"))
    }
    #[test]
    fn test_part2_examples() {
        // aiming for
        // x % 17 == 0
        // x % 13 == 11
        // x % 19 == 16
        // Using (0 ,1, 14) <- picked so that multiplied by product, they hit the target
        // 0 * 13 * 19 + 1 * 17 * 19 + 14 * 17 * 13
        // modulo 17: the 2 right terms are 0
        // modulo 13: the outer terms are 0
        // modulo 10, the 2 left terms are 0
        assert_eq!(3417, part2(&parse_input("0\n17,x,13,19")));
        assert_eq!(1068781, part2(&parse_input("0\n7,13,x,x,59,x,31,19")));
        assert_eq!(754018, part2(&parse_input("0\n67,7,59,61")));
        assert_eq!(779210, part2(&parse_input("0\n67,x,7,59,61")));
        assert_eq!(1261476, part2(&parse_input("0\n67,7,x,59,61")));
        assert_eq!(1202161486, part2(&parse_input("0\n1789,37,47,1889")));
    }
    #[test]
    fn test_part1_example() {
        let input = Input::from(
            "939
7,13,x,x,59,x,31,19",
        );
        assert_eq!(295, part1(&input));
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 119)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 34367022967332)
    }
}
