struct Solution;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
struct FoodRatings {
    food_score: HashMap<String, i32>,
    food_cuisine: HashMap<String, String>,
    cuisine_foods: HashMap<String, HashSet<String>>,
    cuisine_scores: HashMap<String, BTreeMap<i32, BTreeSet<String>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let sz = foods.len();
        let mut food_score: HashMap<String, i32> = HashMap::new();
        for i in 0..sz {
            food_score.insert(foods[i].clone(), ratings[i]);
        }
        let mut food_cuisine: HashMap<String, String> = HashMap::new();
        let mut cuisine_foods: HashMap<String, HashSet<String>> = HashMap::new();
        for i in 0..sz {
            food_cuisine.insert(foods[i].clone(), cuisines[i].clone());
            cuisine_foods
                .entry(cuisines[i].clone())
                .or_default()
                .insert(foods[i].clone());
        }
        let mut cuisine_scores: HashMap<String, BTreeMap<i32, BTreeSet<String>>> = HashMap::new();

        for (c, fs) in cuisine_foods.iter() {
            for f in fs.iter() {
                let score = *food_score.get(f).unwrap();
                cuisine_scores
                    .entry(c.clone())
                    .or_default()
                    .entry(score)
                    .or_default()
                    .insert(f.clone());
            }
        }

        Self {
            food_score,
            food_cuisine,
            cuisine_foods,
            cuisine_scores,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        // let mut cuisine_scores: HashMap<String, BTreeMap<i32, BTreeSet<String>>> = HashMap::new();
        // delete old_rating
        let origin_score = self.food_score.get(&food).unwrap();
        let cuisine = self.food_cuisine.get(&food).unwrap();

        if let Some(scores) = self.cuisine_scores.get_mut(cuisine) {
            let mut should_delete = false;
            if let Some(origin_set) = scores.get_mut(origin_score) {
                origin_set.remove(&food);
                if origin_set.is_empty() {
                    should_delete = true;
                }
            }
            if should_delete {
                scores.remove(origin_score);
            }
        }
        // update new
        self.cuisine_scores
            .entry(cuisine.clone())
            .or_default()
            .entry(new_rating)
            .or_default()
            .insert(food.clone());

        // update score
        self.food_score.insert(food, new_rating);
    }

    fn highest_rated(&self, cuisine: String) -> String {
        // let mut cuisine_scores: HashMap<String, BTreeMap<i32, BTreeSet<String>>> = HashMap::new();
        if let Some(scores) = self.cuisine_scores.get(&cuisine) {
            if let Some((_, max_score_foods)) = scores.iter().rev().next() {
                if let Some(lower_food) = max_score_foods.iter().next() {
                    return lower_food.clone();
                }
            }
        }
        unreachable!()
    }
}

fn main() {}
