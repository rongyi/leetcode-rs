struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        // from small len to bigger
        words.sort_by_key(|w| w.len());
        let word_set: HashSet<String> = words.iter().cloned().collect();
        // k -> v , ending with k, chain len: v
        let mut dp: HashMap<String, i32> = HashMap::new();

        let mut max_chain = 1;
        for ws in words.into_iter() {
            let mut cur_chain = 1;
            for i in 0..ws.len() {
                // yes, strip one to look back, not forward
                let mut striped_one: String = String::with_capacity(ws.len() - 1);
                striped_one.push_str(&ws[0..i]);
                striped_one.push_str(&ws[i + 1..]);

                if let Some(&v) = dp.get(&striped_one) {
                    cur_chain = cur_chain.max(v + 1);
                }
            }

            dp.insert(ws, cur_chain);
            max_chain = max_chain.max(cur_chain);
        }

        max_chain
    }
}

fn main() {}
