#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let sz = favorite_companies.len();
        let mut favi_set: Vec<HashSet<String>> = vec![HashSet::new(); sz];
        for (i, lst) in favorite_companies.iter().enumerate() {
            for cmp in lst.iter() {
                favi_set[i].insert(cmp.to_string());
            }
        }
        let mut ret: Vec<usize> = Vec::new();
        for i in 0..sz {
            let mut no_subset = true;
            for j in 0..sz {
                if i == j {
                    continue;
                }
                if favi_set[i].is_subset(&favi_set[j]) {
                    no_subset = false;
                }
            }
            if no_subset {
                ret.push(i);
            }
        }

        ret.into_iter().map(|i| i as i32).collect()
    }
}

fn main() {}
