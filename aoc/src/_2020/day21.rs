use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let mut ingredients = HashMap::<String, usize>::new();
        let mut allergens = HashMap::<String, HashMap<String, usize>>::new();
        let mut allergen_counts = HashMap::<String, usize>::new();
        for food in self.parse_input() {
            for ingredient in food.ingredients.iter() {
                *(ingredients.entry(ingredient.clone()).or_insert(0)) += 1;
            }
            for allergen in food.allergens.iter() {
                *(allergen_counts.entry(allergen.clone()).or_insert(0)) += 1;
                for ingredient in food.ingredients.iter() {
                    *(allergens
                        .entry(allergen.clone())
                        .or_insert_with(HashMap::<String, usize>::new)
                        .entry(ingredient.clone())
                        .or_insert(0)) += 1;
                }
            }
        }
        let mut count = 0;
        'ING: for (ingredient, ingredient_count) in ingredients.iter() {
            for (allergen, ingredient_counts) in allergens.iter() {
                if ingredient_counts.get(ingredient).unwrap_or(&0)
                    == allergen_counts.get(allergen).unwrap_or(&0)
                {
                    continue 'ING;
                }
            }
            for (_, ingredient_counts) in allergens.iter_mut() {
                ingredient_counts.remove(ingredient);
            }
            count += ingredient_count;
        }
        count.to_string()
    }

    fn run_b(&self) -> String {
        let mut ingredients = HashMap::<String, usize>::new();
        let mut allergens = HashMap::<String, HashMap<String, usize>>::new();
        let mut allergen_counts = HashMap::<String, usize>::new();
        for food in self.parse_input() {
            for ingredient in food.ingredients.iter() {
                *(ingredients.entry(ingredient.clone()).or_insert(0)) += 1;
            }
            for allergen in food.allergens.iter() {
                *(allergen_counts.entry(allergen.clone()).or_insert(0)) += 1;
                for ingredient in food.ingredients.iter() {
                    *(allergens
                        .entry(allergen.clone())
                        .or_insert_with(HashMap::<String, usize>::new)
                        .entry(ingredient.clone())
                        .or_insert(0)) += 1;
                }
            }
        }
        'ING: for (ingredient, _) in ingredients.iter() {
            for (allergen, ingredient_counts) in allergens.iter_mut() {
                let c = ingredient_counts.get(ingredient).unwrap_or(&0);
                if c == allergen_counts.get(allergen).unwrap_or(&0) {
                    continue 'ING;
                } else if c < allergen_counts.get(allergen).unwrap() {
                    ingredient_counts.remove(ingredient);
                }
            }
            for (_, ingredient_counts) in allergens.iter_mut() {
                ingredient_counts.remove(ingredient);
            }
        }
        let mut found = Vec::<(String, String)>::new();
        let mut exclude = HashSet::new();
        allergens
            .iter()
            .sorted_by(|(_, a), (_, b)| a.len().cmp(&b.len()))
            .for_each(|(p, set)| {
                let cands: Vec<_> = set.iter().filter(|(k, _)| !exclude.contains(k)).collect();
                println!("{} {:?}", p, cands);
                exclude.insert(cands[0].0);
                found.push((p.clone(), cands[0].0.clone()));
            });

        found.sort_by(|a, b| a.0.cmp(&b.0));
        found
            .iter()
            .map(|(_, i)| i.clone())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl Runner {
    fn parse_input(&self) -> Vec<Food> {
        self.input
            .trim()
            .lines()
            .map(|l| {
                let parts: Vec<_> = l.trim_end_matches(')').split(" (contains ").collect();
                Food {
                    ingredients: parts[0].split(' ').map(|s| s.to_owned()).collect(),
                    allergens: parts[1].split(", ").map(|s| s.to_owned()).collect(),
                }
            })
            .collect()
    }
}

struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}
