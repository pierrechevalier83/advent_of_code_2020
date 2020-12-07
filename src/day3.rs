use aoc_runner_derive::{aoc, aoc_generator};

use std::{collections::HashSet, str::FromStr};

struct Row {
    positions: HashSet<usize>,
    period: usize,
}

impl FromStr for Row {
    type Err = String;

    // Parse a string with format: "...###.#..#.#"
    // where "#" represents an object
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            positions: s
                .chars()
                .enumerate()
                .filter_map(|(i, x)| if x == '#' { Some(i) } else { None })
                .collect(),
            period: s.chars().count(),
        })
    }
}

impl Row {
    fn contains_object(&self, position: usize) -> bool {
        self.period != 0 && self.positions.contains(&(position % self.period))
    }
}

struct Matrix {
    rows: Vec<Row>,
}

impl FromStr for Matrix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Row> = s
            .split_terminator('\n')
            .map(|s| Row::from_str(s))
            .collect::<Result<Vec<Row>, String>>()?;
        Ok(Self { rows })
    }
}

impl Matrix {
    fn count_objects_on_slope(&self, right: usize, down: usize) -> usize {
        let mut row_index = 0;
        let mut col_index = 0;
        self.rows
            .iter()
            .enumerate()
            .filter(|(index, row)| {
                if *index == row_index {
                    let contains_object = row.contains_object(col_index);
                    row_index += down;
                    col_index += right;
                    contains_object
                } else {
                    false
                }
            })
            .count()
    }
}

#[aoc_generator(day3)]
fn parse_input(data: &str) -> Matrix {
    Matrix::from_str(data).unwrap()
}

#[aoc(day3, part1)]
fn part1(forest: &Matrix) -> usize {
    forest.count_objects_on_slope(3, 1)
}

fn product_of_trees_on_slopes(slopes: &[(usize, usize)], forest: &Matrix) -> usize {
    slopes
        .iter()
        .map(|(right, down)| forest.count_objects_on_slope(*right, *down))
        .product()
}

#[aoc(day3, part2)]
fn part2(forest: &Matrix) -> usize {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    product_of_trees_on_slopes(&slopes, &forest)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SMALL_FOREST: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    #[test]
    fn test_count_objects_on_slope() {
        let forest = Matrix::from_str(SMALL_FOREST).unwrap();
        assert_eq!(7, forest.count_objects_on_slope(3, 1))
    }
    #[test]
    fn test_product_of_trees_on_slopes() {
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let forest = Matrix::from_str(SMALL_FOREST).unwrap();
        assert_eq!(336, product_of_trees_on_slopes(&slopes, &forest))
    }
    fn input() -> Matrix {
        parse_input(include_str!("../input/2020/day3.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 195)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 3772314000)
    }
}
