use itertools::Itertools;
use petgraph::{algo, prelude::*};
use std::collections::HashMap;
use std::str::FromStr;

fn parse_input() -> BagRules {
    let data = include_str!("input.txt");
    BagRules::from_str(data).unwrap()
}

struct BagRules {
    nodes: HashMap<String, NodeIndex>,
    graph: DiGraph<String, u32>,
}

fn digraph_from_weighted_edges(
    weighted_edges: &[(String, String, u32)],
) -> (HashMap<String, NodeIndex>, DiGraph<String, u32>) {
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
                let container = container.strip_suffix(" bags").unwrap();
                all_content.split(", ").map(move |content| {
                    let content = content
                        .strip_suffix(" bags.")
                        .or_else(|| content.strip_suffix(" bag."))
                        .or_else(|| content.strip_suffix(" bags"))
                        .or_else(|| content.strip_suffix(" bag"))
                        .unwrap();
                    let (quantity, bag) = content
                        .splitn(2, ' ')
                        .map(|s| s.to_string())
                        .collect_tuple()
                        .unwrap();
                    Ok((
                        container.to_string(),
                        bag,
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
    fn count_types_of_bags_which_can_contain(&self, target: &str) -> usize {
        let target = self.nodes[&target.to_string()];
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
    fn count_bags_which_must_be_contained(&self, target: &str) -> usize {
        let target = self.nodes[&target.to_string()];
        self.accumulate_edge_weights(target)
    }
}

fn part1() -> usize {
    parse_input().count_types_of_bags_which_can_contain("shiny gold")
}

fn part2() -> usize {
    parse_input().count_bags_which_must_be_contained("shiny gold")
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
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
        assert_eq!(4, rules.count_types_of_bags_which_can_contain("shiny gold"));
        assert_eq!(7, rules.count_types_of_bags_which_can_contain("faded blue"));
        assert_eq!(0, rules.count_types_of_bags_which_can_contain("light red"));
        assert_eq!(32, rules.count_bags_which_must_be_contained("shiny gold"));
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 222)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 13264)
    }
}
