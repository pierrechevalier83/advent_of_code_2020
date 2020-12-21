use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

// HashSet
// dairy => mxmxvkd kfcds sqhjc nhms
// fish => mxmxvkd kfcds sqhjc nhms
// dairy => intersect with trh fvjkl sbzzf mxmxvkd
// dairy => mxmxvkd
// not dairy: everything else (in big Vec of ingredients, with duplicates))
// mapped => mxmxvkd
//
// fish => intersect with sqjhc mxmxvkd sbzzf
// fish => mxmxvkd sqhjc
//   remove perfect mappings
// fish => sqhjc
// mapped => mxmxvkd, sqhjc
//
// soy => sqjhc, fvjkl
// soy => fvjkl
// mapped => mxmxvkd, sqhjc, fvjkl
//
// fish => sqjhc mxmxvkd sbzzf
// sbzzf
// mapped => mxmxvkd, sqhjc, fvjkl, sbzzf
//
// Build all intersection sets
// Find one that has size one when filtered with mapped
// Add mapping

type Food = String;
type Ingredient = String;

// Note: if performance isn't satisfying, we could keep one mapping of ingredients and foods to
// indices and operate on ints instead of string everywhere, and replace some BTreeMaps with
// Vecs where the keys are the indices at which the elements are found.
#[derive(Debug)]
struct FoodList {
    // mapping from ingredients to foods that may contain it
    may_contain: BTreeMap<Ingredient, HashSet<Food>>,
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
    fn num_unmapped_ingredients(&self) -> usize {
        let mut mapped_foods = HashSet::new();
        while let Some(food) = self.may_contain.iter().find_map(|(_ingredient, foods)| {
            let only_possible_foods = foods.difference(&mapped_foods).collect::<Vec<_>>();
            if only_possible_foods.len() == 1 {
                Some(only_possible_foods[0].clone())
            } else {
                None
            }
        }) {
            mapped_foods.insert(food);
        }
        self.counts
            .iter()
            .filter(|(food, _)| mapped_foods.get(food.clone()).is_none())
            .map(|(_, count)| count)
            .sum()
    }
}

#[aoc_generator(day21)]
fn parse_input(s: &str) -> FoodList {
    s.into()
}

#[aoc(day21, part1)]
fn part1(foods: &FoodList) -> usize {
    foods.num_unmapped_ingredients()
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
}
