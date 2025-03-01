#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut result = Vec::with_capacity(names.len());
        let mut name_counts: HashMap<String, i32> = HashMap::new();

        for name in names {
            if !name_counts.contains_key(&name) {
                // Case 1: Name is unique so far
                result.push(name.clone());
                name_counts.insert(name, 1);
            } else {
                // Case 2: Name already exists, find a new name with suffix
                let mut k = *name_counts.get(&name).unwrap();
                let mut new_name = format!("{}({})", name, k);

                // Find the smallest k where name(k) is unique
                while name_counts.contains_key(&new_name) {
                    k += 1;
                    new_name = format!("{}({})", name, k);
                }

                // Update the count for the original name
                name_counts.insert(name, k + 1);
                // Add the new name with count 1
                name_counts.insert(new_name.clone(), 1);
                result.push(new_name);
            }
        }

        result
    }
}

fn main() {}
