#![allow(dead_code)]

struct Solution;
use std::collections::{HashMap, HashSet};

struct ThroneInheritance {
    king: String,
    family_tree: HashMap<String, Vec<String>>,
    dead: HashSet<String>,
}

impl ThroneInheritance {
    fn new(king: String) -> Self {
        let mut family_tree = HashMap::new();
        family_tree.insert(king.clone(), Vec::new());

        ThroneInheritance {
            king,
            family_tree,
            dead: HashSet::new(),
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        self.family_tree
            .entry(parent_name)
            .or_insert(Vec::new())
            .push(child_name.clone());

        // self.family_tree.entry(child_name).or_insert(Vec::new());
    }

    fn death(&mut self, name: String) {
        self.dead.insert(name);
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        let mut result = Vec::new();
        self.dfs(&self.king, &mut result);
        result
    }

    fn dfs(&self, current: &String, result: &mut Vec<String>) {
        // Add current person to inheritance order if not dead
        if !self.dead.contains(current) {
            result.push(current.clone());
        }

        // Recursively traverse all children
        if let Some(children) = self.family_tree.get(current) {
            for child in children {
                self.dfs(child, result);
            }
        }
    }
}
fn main() {}
