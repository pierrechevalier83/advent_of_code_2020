use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use petgraph::{algo, prelude::*};
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::EnumString;

#[aoc_generator(day7)]
fn parse_input(data: &str) -> BagRules {
    BagRules::from_str(data).unwrap()
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, EnumString)]
#[strum(serialize_all = "snake_case")]
enum Adjective {
    Bright,
    Clear,
    Dark,
    Dim,
    Dotted,
    Drab,
    Dull,
    Faded,
    Mirrored,
    Muted,
    Light,
    Pale,
    Plaid,
    Posh,
    Shiny,
    Striped,
    Vibrant,
    Wavy,
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, EnumString)]
#[strum(serialize_all = "snake_case")]
enum Color {
    Aqua,
    Beige,
    Black,
    Blue,
    Bronze,
    Brown,
    Chartreuse,
    Coral,
    Crimson,
    Cyan,
    Gold,
    Gray,
    Green,
    Fuchsia,
    Indigo,
    Lavender,
    Lime,
    Magenta,
    Maroon,
    Olive,
    Orange,
    Plum,
    Purple,
    Red,
    Salmon,
    Silver,
    Tan,
    Teal,
    Tomato,
    Turquoise,
    Violet,
    White,
    Yellow,
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct Bag {
    adjective: Adjective,
    color: Color,
}

impl FromStr for Bag {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (adjective, color) = s
            .strip_suffix(" bags.")
            .or_else(|| s.strip_suffix(" bag."))
            .or_else(|| s.strip_suffix(" bags"))
            .or_else(|| s.strip_suffix(" bag"))
            .unwrap()
            .splitn(2, ' ')
            .collect_tuple()
            .unwrap();
        Ok(Bag {
            adjective: Adjective::from_str(adjective)
                .map_err(|e| format!("{} for {}", e, adjective))?,
            color: Color::from_str(color).map_err(|e| format!("{} for {}", e, color))?,
        })
    }
}

struct BagRules {
    nodes: HashMap<Bag, NodeIndex>,
    graph: DiGraph<Bag, u32>,
}

fn digraph_from_weighted_edges(
    weighted_edges: &[(Bag, Bag, u32)],
) -> (HashMap<Bag, NodeIndex>, DiGraph<Bag, u32>) {
    let mut graph = DiGraph::new();
    let mut nodes = HashMap::new();
    for (input, output, weight) in weighted_edges {
        let input_idx = *nodes
            .entry(input.clone())
            .or_insert_with(|| graph.add_node(input.clone()));
        let output_idx = *nodes
            .entry(output.clone())
            .or_insert_with(|| graph.add_node(output.clone()));
        graph.add_edge(input_idx, output_idx, *weight);
    }
    (nodes, graph)
}

impl FromStr for BagRules {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let weighted_edges = s
            .split_terminator('\n')
            .filter(|line| !line.contains("no other bags."))
            .map(|line| line.split(" contain ").collect_tuple().unwrap())
            .flat_map(|(container, all_content)| {
                all_content.split(", ").map(move |content| {
                    let (quantity, content) = content.splitn(2, ' ').collect_tuple().unwrap();

                    Ok((
                        Bag::from_str(container)?,
                        Bag::from_str(content)?,
                        quantity.parse::<u32>().map_err(|_| {
                            format!("Couldn't parse \"{}\" as a bag quantity", quantity)
                        })?,
                    ))
                })
            })
            .collect::<Result<Vec<_>, Self::Err>>()?;
        let (nodes, graph) = digraph_from_weighted_edges(&weighted_edges);
        Ok(Self { nodes, graph })
    }
}

impl BagRules {
    fn count_types_of_bags_which_can_contain(&self, target: &Bag) -> usize {
        let target = self.nodes[target];
        self.graph
            .node_indices()
            .filter(|node| *node != target)
            .filter(|node| algo::has_path_connecting(&self.graph, *node, target, None))
            .count()
    }
    fn accumulate_edge_weights(&self, node: NodeIndex) -> usize {
        self.graph
            .edges_directed(node, Direction::Outgoing)
            .map(|edge| *edge.weight() as usize * (1 + self.accumulate_edge_weights(edge.target())))
            .sum()
    }
    fn count_bags_which_must_be_contained(&self, target: &Bag) -> usize {
        let target = self.nodes[target];
        self.accumulate_edge_weights(target)
    }
}

const SHINY_GOLD: Bag = Bag {
    adjective: Adjective::Shiny,
    color: Color::Gold,
};

#[aoc(day7, part1)]
fn part1(bag_rules: &BagRules) -> usize {
    bag_rules.count_types_of_bags_which_can_contain(&SHINY_GOLD)
}

#[aoc(day7, part2)]
fn part2(bag_rules: &BagRules) -> usize {
    bag_rules.count_bags_which_must_be_contained(&SHINY_GOLD)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &'static str =
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    #[test]
    fn example() {
        let rules = BagRules::from_str(EXAMPLE_INPUT).unwrap();
        assert_eq!(4, rules.count_types_of_bags_which_can_contain(&SHINY_GOLD));
        assert_eq!(32, rules.count_bags_which_must_be_contained(&SHINY_GOLD));
    }
    fn input() -> BagRules {
        parse_input(include_str!("../input/2020/day7.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 222)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 13264)
    }
}
