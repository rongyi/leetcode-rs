
struct Solution;

use std::collections::{BTreeMap, HashMap};
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut index1: HashMap<String, usize> = HashMap::new();
        for (i, s) in list1.iter().enumerate() {
            index1.insert(s.clone(), i);
        }
        let mut group: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        for (j, s) in list2.iter().enumerate() {
            if let Some(&i) = index1.get(s) {
                group.entry(i + j).or_insert(Vec::new()).push(s.clone());
            }
        }
        group.first_key_value().unwrap().1.to_owned()
    }
}

fn main() {}
