use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let foods = parse_input(&content)?;

    let safe_ingredients = find_safe_ingredients(&foods);
    let safe_ingredient_occurences = count_ingredient_occurences(&safe_ingredients, &foods);
    println!(
        "Safe ingredients occur {} times",
        safe_ingredient_occurences
    );

    let ordered_dangerous_ingredients = get_ordered_dangerous_ingredients(&foods)?;
    let canonical_dangerous_ingredients_list: Vec<&str> = ordered_dangerous_ingredients
        .iter()
        .map(|(ing, _)| *ing)
        .collect();
    println!(
        "The canonical dangerous ingredient list is {}",
        canonical_dangerous_ingredients_list.join(",")
    );

    Ok(())
}

fn count_ingredient_occurences(ingredients: &HashSet<&str>, foods: &[Food]) -> usize {
    foods
        .iter()
        .flat_map(|food| &food.ingredients)
        .filter(|ing| ingredients.contains(*ing))
        .count()
}

fn find_safe_ingredients<'a>(foods: &[Food<'a>]) -> HashSet<&'a str> {
    let may_contain = create_may_contain(foods);

    let mut ingredients: HashSet<&str> = foods
        .iter()
        .flat_map(|food| &food.ingredients)
        .copied()
        .collect();

    for allergen_ingr in may_contain.values() {
        ingredients.retain(|ing| !allergen_ingr.contains(ing));
    }

    ingredients
}

fn get_ordered_dangerous_ingredients<'a>(
    foods: &[Food<'a>],
) -> Result<Vec<(&'a str, &'a str)>, String> {
    let mut may_contain = create_may_contain(foods);
    let mut contains: HashMap<&'a str, &'a str> = HashMap::with_capacity(may_contain.len());

    let mut found = true;
    while found {
        found = false;
        let mut found_ingredient: Option<&str> = None;
        for (allergen, ingredients) in may_contain.iter() {
            if ingredients.len() == 1 {
                let ingredient = ingredients.iter().next().unwrap();
                contains.insert(ingredient, allergen);
                found = true;
                found_ingredient = Some(ingredient);
            }
        }
        if let Some(ingredient) = found_ingredient {
            for ings in may_contain.values_mut() {
                ings.remove(ingredient);
            }
        }
    }

    if contains.len() != may_contain.len() {
        return Err("Unable to find an ingredient for all allergens".to_owned());
    }

    let mut result: Vec<(&str, &str)> = contains
        .iter()
        .map(|(ingr, allerg)| (*ingr, *allerg))
        .collect();
    result.sort_unstable_by_key(|(_, allergen)| *allergen);
    Ok(result)
}

fn create_may_contain<'a>(foods: &[Food<'a>]) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut may_contain: HashMap<&str, HashSet<&str>> = HashMap::with_capacity(foods.len() * 2);
    for food in foods {
        for allergen in &food.allergens {
            may_contain
                .entry(allergen)
                .and_modify(|ingr| *ingr = ingr.intersection(&food.ingredients).copied().collect())
                .or_insert_with(|| food.ingredients.clone());
        }
    }
    may_contain
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

fn parse_input(input: &str) -> Result<Vec<Food>, String> {
    input.split_terminator('\n').map(parse_food).collect()
}

fn parse_food(line: &str) -> Result<Food, String> {
    let mut split = line.splitn(2, " (contains ");
    let ingredients: HashSet<&str> = split
        .next()
        .ok_or_else(|| format!("Expected ingredients in line '{}'", line))?
        .split_whitespace()
        .collect();
    let allergens: HashSet<&str> = split
        .next()
        .ok_or_else(|| format!("Expected allergens in line '{}'", line))?
        .trim_end_matches(')')
        .split(", ")
        .collect();

    Ok(Food {
        ingredients,
        allergens,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_safe_ingredients_works_for_example() {
        // given
        let input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let foods = parse_input(input).expect("Expected example input to parse");

        // when
        let result = find_safe_ingredients(&foods);
        let safe_ingredient_occurences = count_ingredient_occurences(&result, &foods);

        // then
        assert_eq!(result.len(), 4);
        assert!(result.contains("kfcds"));
        assert!(result.contains("nhms"));
        assert!(result.contains("sbzzf"));
        assert!(result.contains("trh"));

        assert_eq!(safe_ingredient_occurences, 5);
    }

    #[test]
    fn get_ordered_dangerous_ingredients_works_for_example() {
        // given
        let input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let foods = parse_input(input).expect("Expected example input to parse");

        // when
        let result = get_ordered_dangerous_ingredients(&foods).expect("Expected success");

        // then
        assert_eq!(
            result,
            &[("mxmxvkd", "dairy"), ("sqjhc", "fish"), ("fvjkl", "soy")]
        );
    }
}
