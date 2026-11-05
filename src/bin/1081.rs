struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut ret: Vec<char> = Vec::new();
        let mut cnt: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            *cnt.entry(c).or_insert(0) += 1;
        }
        let mut used = HashMap::new();

        for c in s.chars() {
            cnt.entry(c).and_modify(|num| *num -= 1);
            if used.contains_key(&c) {
                continue;
            }
            used.insert(c, true);

            while !ret.is_empty()
                && *ret.last().unwrap() > c
                && *cnt.get(ret.last().unwrap()).unwrap_or(&0) > 0
            {
                used.remove(ret.last().unwrap());
                ret.pop();
            }

            ret.push(c);
        }

        ret.into_iter().collect()
    }
}

fn main() {}
