struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut cnt: HashMap<String, i32> = HashMap::new();
        for w in words.iter() {
            *cnt.entry(w.to_string()).or_default() += 1;
        }
        let mut total = 0;
        let mut has_odd = false;
        let keys: Vec<String> = cnt.keys().cloned().collect();

        for w in keys.iter() {
            let wb = w.as_bytes();
            if wb[0] == wb[1] {
                let cur_pair_cnt = *cnt.get(w).unwrap_or(&0);
                // odd count
                if cur_pair_cnt % 2 == 1 {
                    has_odd = true;
                }
                // contribute even pair , so total len contribute is: evenpair * 2 * 2
                total += (cur_pair_cnt / 2) * 2 * 2;
            } else {
                let cur_cnt = *cnt.get(w).unwrap_or(&0);
                let wp: String = w.chars().rev().collect();
                let opp_cnt = *cnt.get(&wp).unwrap_or(&0);

                total += cur_cnt.min(opp_cnt) * 2 * 2;

                cnt.remove(&wp);
            }
        }
        if has_odd {
            total += 2;
        }

        total
    }
}

fn main() {}
