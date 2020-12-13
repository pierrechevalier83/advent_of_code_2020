use aoc_runner_derive::{aoc, aoc_generator};

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
    fn time_to_wait(&self, bus: usize) -> usize {
        bus - self.earliest % bus
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

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Input {
        parse_input(include_str!("../input/2020/day13.txt"))
    }
    #[test]
    fn test_example() {
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
}
