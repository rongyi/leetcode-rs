#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        use std::collections::{HashMap, HashSet};

        // Create a map to store times for each person
        let mut time_map: HashMap<String, Vec<i32>> = HashMap::new();

        // Process all entries
        for i in 0..key_name.len() {
            let name = &key_name[i];
            let time = &key_time[i];

            // Parse the time string (HH:MM) into minutes since start of day
            let parts: Vec<&str> = time.split(':').collect();
            let hours: i32 = parts[0].parse().unwrap();
            let minutes: i32 = parts[1].parse().unwrap();
            let total_minutes = hours * 60 + minutes;

            // Add the time to the person's list
            time_map
                .entry(name.clone())
                .or_insert(Vec::new())
                .push(total_minutes);
        }

        // Find people who have 3 or more keycard uses within 1 hour
        let mut alerted: HashSet<String> = HashSet::new();

        for (name, times) in time_map.iter_mut() {
            // Sort the times for efficient comparison
            times.sort();

            // Check if there are 3 uses within 1 hour

            // The saturating_sub prevents underflow if times.len() < 2
            // It ensures the loop only runs when we have at least 3 entries to compare
            for i in 0..times.len().saturating_sub(2) {
                if times[i + 2] - times[i] <= 60 {
                    alerted.insert(name.clone());
                    break;
                }
            }
        }

        // Convert the HashSet to a sorted Vec
        let mut result: Vec<String> = alerted.into_iter().collect();
        result.sort();

        result
    }
}

fn main() {}
