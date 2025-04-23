use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct Ingredient(String);

#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct Allergen(String);

impl std::fmt::Display for Allergen {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(Debug)]
pub struct Food {
	ingredients: Vec<Ingredient>,
	allergens: Vec<Allergen>,
}

impl core::str::FromStr for Food {
	type Err = ();

	fn from_str(str: &str) -> Result<Food, ()> {
		let (ingredients_list, allergens_list): (&str, &str) = {
			let mut split = str.split(" (contains ");
			let ingredients_list: &str = split.next().unwrap();
			let allergens_list: &str = split.next().unwrap().split(')').next().unwrap();

			(ingredients_list, allergens_list)
		};

		let ingredients: Vec<Ingredient> = ingredients_list
			.split(' ')
			.map(str::to_owned)
			.map(Ingredient)
			.collect();
		let allergens: Vec<Allergen> = allergens_list
			.split(", ")
			.map(str::to_owned)
			.map(Allergen)
			.collect();

		Ok(Food {
			ingredients,
			allergens,
		})
	}
}

pub type Intermediate = (Vec<Food>, HashMap<Allergen, Ingredient>);
pub type Output = String;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	let foods: Vec<Food> = input
		.lines()
		.map(|line| line.parse::<Food>().expect("failed to parse food"))
		.collect();

	let mut candidates: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();

	for food in &foods {
		let food_ingredients: HashSet<Ingredient> = food.ingredients.iter().cloned().collect();

		for allergen in &food.allergens {
			let allergen: Allergen = allergen.clone();

			let possible_ingredients = candidates
				.entry(allergen)
				.or_insert_with(|| food_ingredients.clone());

			*possible_ingredients = &*possible_ingredients & &food_ingredients;
		}
	}

	let mut allergens: HashMap<Allergen, Ingredient> = HashMap::new();

	while let Some((allergen, _)) = candidates.iter().find(|(_, set)| set.len() == 1) {
		let allergen_set: &HashSet<Ingredient> = candidates.get(allergen).unwrap();

		let ingredient: Ingredient = allergen_set.iter().next().unwrap().clone();
		allergens.insert(allergen.clone(), ingredient.clone());

		for set in candidates.values_mut() {
			set.remove(&ingredient);
		}
	}

	Ok((foods, allergens))
}

pub fn part_one((foods, allergens): &Intermediate) -> Option<Output> {
	let all_allergens = allergens.values().collect::<HashSet<_>>();

	Some(
		foods
			.iter()
			.flat_map(|food| food.ingredients.clone())
			.filter(|ingredient| !all_allergens.contains(ingredient))
			.count()
			.to_string(),
	)
}

pub fn part_two((_, allergens): &Intermediate) -> Option<Output> {
	let allergens: BTreeMap<Allergen, Ingredient> = allergens.clone().drain().collect();

	let all_ingredients = allergens
		.values()
		.map(|ingredient| ingredient.0.as_str())
		.collect::<Vec<&str>>();

	Some(all_ingredients.join(","))
}

daocutil::generate_solver!(solve, =>, self);
