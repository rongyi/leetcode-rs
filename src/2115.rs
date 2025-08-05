
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut supplies_set: HashSet<String> = supplies.into_iter().collect();
        let mut visited: HashSet<String> = HashSet::new();

        loop {
            let mut produced_new_recipe = false;

            for (i, r) in recipes.iter().enumerate() {
                if visited.contains(r) {
                    continue;
                }
                let cur_ingrd: HashSet<String> =
                    ingredients[i].iter().map(|s| s.to_string()).collect();
                // all contained in supplies?
                // ok, produced this recipe
                if cur_ingrd.iter().all(|e| supplies_set.contains(e)) {
                    visited.insert(r.to_string());
                    // add to supply
                    supplies_set.insert(r.to_string());
                    produced_new_recipe = true;
                }
            }

            if !produced_new_recipe {
                break;
            }
        }

        // any order
        visited.into_iter().collect()
    }
}
fn main() {}
