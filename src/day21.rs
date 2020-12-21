use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

type Alergen = String;
type Food = String;

// Note: if performance isn't satisfying, we could keep one mapping of ingredients and foods to
// indices and operate on ints instead of string everywhere, and replace some BTreeMaps with
// Vecs where the keys are the indices at which the elements are found.
#[derive(Debug)]
struct FoodList {
    // mapping from alergens to foods that may contain it
    may_contain: BTreeMap<Alergen, HashSet<Food>>,
    // How many times did we see each ingredient
    counts: BTreeMap<Food, usize>,
}

impl From<&str> for FoodList {
    fn from(s: &str) -> Self {
        let mut may_contain = BTreeMap::new();
        let mut counts = BTreeMap::new();
        s.split_terminator("\n").for_each(|line| {
            let (foods, ingredients) = line
                .replace(")", "")
                .replace(",", "")
                .split(" (contains ")
                .map(|list| list.split(" ").map(|s| s.to_string()).collect::<Vec<_>>())
                .collect_tuple()
                .unwrap();
            let foods = foods
                .into_iter()
                .inspect(|food| {
                    let count = counts.entry(food.clone()).or_insert(0);
                    *count += 1;
                })
                .map(|s| s.clone())
                .collect::<HashSet<_>>();
            for ingredient in ingredients {
                let may_contain = may_contain.entry(ingredient).or_insert(foods.clone());
                *may_contain = may_contain.intersection(&foods).cloned().collect();
            }
        });
        Self {
            may_contain,
            counts,
        }
    }
}

impl FoodList {
    fn num_unmapped_ingredients_and_mappings(&self) -> (usize, Vec<(Alergen, Food)>) {
        let mut mapped_foods = HashSet::new();
        let mut alergen_mappings = Vec::<(Alergen, Food)>::new();
        while let Some(food) = self.may_contain.iter().find_map(|(alergen, foods)| {
            let only_possible_foods = foods.difference(&mapped_foods).collect::<Vec<_>>();
            if only_possible_foods.len() == 1 {
                let food = only_possible_foods[0];
                alergen_mappings.push((alergen.clone(), food.clone()));
                Some(food.clone())
            } else {
                None
            }
        }) {
            mapped_foods.insert(food);
        }
        let num_unmapped_ingredients = self
            .counts
            .iter()
            .filter(|(food, _)| mapped_foods.get(food.clone()).is_none())
            .map(|(_, count)| count)
            .sum();
        (num_unmapped_ingredients, alergen_mappings)
    }
}

#[aoc_generator(day21)]
fn parse_input(s: &str) -> FoodList {
    s.into()
}

#[aoc(day21, part1)]
fn part1(foods: &FoodList) -> usize {
    foods.num_unmapped_ingredients_and_mappings().0
}

#[aoc(day21, part2)]
fn part2(foods: &FoodList) -> String {
    let mut mappings = foods.num_unmapped_ingredients_and_mappings().1;
    mappings.sort_by(|(left_alergen, _), (right_alergen, _)| left_alergen.cmp(right_alergen));
    mappings
        .iter()
        .map(|(_, food)| food.to_owned())
        .intersperse(",".to_string())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> FoodList {
        parse_input(include_str!("../input/2020/day21.txt"))
    }
    const EXAMPLE: &'static str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    #[test]
    fn test_example() {
        assert_eq!(5, part1(&parse_input(EXAMPLE)))
    }
    #[test]
    fn test_part1() {
        assert_eq!(2150, part1(&input()))
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            "vpzxk,bkgmcsx,qfzv,tjtgbf,rjdqt,hbnf,jspkl,hdcj",
            part2(&input())
        )
    }
}
