use std::collections::HashMap;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let (count, ..) = self.init();
        count
    }

    fn run_b(&self) -> String {
        let (_, mut allergens, allergen_counts) = self.init();
        let mut found = Vec::<(String, String)>::new();
        while found.len() < allergen_counts.len() {
            for (a, is) in allergens.clone().iter() {
                for (i, c) in is.iter() {
                    if c < allergen_counts.get(a).unwrap() {
                        allergens.entry(a.clone()).and_modify(|e| {
                            e.remove(i).unwrap_or_else(|| {
                                panic!("Unable to remove entry {} {} (1)", a, i)
                            });
                        });
                    }
                }
                if is.len() == 1 {
                    for i in is.keys() {
                        found.push((a.clone(), i.clone()));
                        for k in allergens.clone().keys() {
                            allergens.entry(k.clone()).and_modify(|e| {
                                if e.contains_key(i) {
                                    e.remove(i).unwrap_or_else(|| {
                                        panic!("Unable to remove entry {} {} (2): {:?}", a, i, k)
                                    });
                                }
                            });
                        }
                    }
                    allergens.remove(a);
                }
            }
        }

        found.sort_by(|a, b| a.0.cmp(&b.0));
        found
            .iter()
            .map(|(_, i)| i.clone())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl Runner {
    #[allow(clippy::type_complexity)]
    fn init(
        &self,
    ) -> (
        String,
        HashMap<String, HashMap<String, usize>>,
        HashMap<String, usize>,
    ) {
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
        (count.to_string(), allergens, allergen_counts)
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "21"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "21_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("5"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("mxmxvkd,sqjhc,fvjkl"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("2569"));
    }

    #[test]
    fn real_b() {
        assert_eq!(
            new().run_b(),
            String::from("vmhqr,qxfzc,khpdjv,gnrpml,xrmxxvn,rfmvh,rdfr,jxh")
        );
    }
}
