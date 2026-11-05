struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut cnt: HashMap<char, i32> = HashMap::new();
        let mut used: HashMap<char, bool> = HashMap::new();

        for c in s.chars() {
            *cnt.entry(c).or_insert(0) += 1;
        }
        let mut ret: Vec<char> = Vec::new();
        for c in s.chars() {
            cnt.entry(c).and_modify(|v| *v -= 1);
            if *used.get(&c).unwrap_or(&false) {
                continue;
            }

            // we meet a small value and check we still have more
            // value after this small value, so be nice, we pop
            // this bigger value from stack, and let the smaller
            // vlaue in, and then, this bigger one will go into stack
            // again, every one is happy, be nice!
            while !ret.is_empty()
                && *ret.last().unwrap() > c
                && *cnt.get(ret.last().unwrap()).unwrap_or(&0) > 0
            {
                used.insert(*ret.last().unwrap(), false);
                ret.pop();
            }

            used.insert(c, true);
            ret.push(c);
        }

        ret.into_iter().collect()
    }
}

fn main() {}
