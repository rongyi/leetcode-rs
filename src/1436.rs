#![allow(dead_code)]

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut cities: HashSet<String> = HashSet::new();
        for cur_path in paths.iter() {
            graph
                .entry(cur_path[0].clone())
                .or_default()
                .push(cur_path[1].clone());
            cities.insert(cur_path[0].clone());
            cities.insert(cur_path[1].clone());
        }

        for c in cities.iter() {
            if !graph.contains_key(c) {
                return c.clone();
            }
        }

        "".to_string()
    }
}

fn main() {}
