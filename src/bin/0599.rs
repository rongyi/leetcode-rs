struct Solution;

use std::{
    collections::{BTreeMap, HashMap},
    usize,
};
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut index1: HashMap<String, usize> = HashMap::new();

        for (i, s) in list1.iter().enumerate() {
            index1.insert(s.clone(), i);
        }

        let mut min_idx = usize::MAX;
        let mut min_lst = vec![];
        for (j, s) in list2.iter().enumerate() {
            if let Some(&i) = index1.get(s) {
                let cur_idx = i + j;

                if cur_idx < min_idx {
                    min_idx = cur_idx;
                    min_lst.clear();
                    min_lst.push(s.clone());
                } else if cur_idx == min_idx {
                    min_lst.push(s.clone());
                }
            }
        }

        min_lst
    }
}

fn main() {}
