
struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        // from small len to bigger
        words.sort_by_key(|w| w.len());
        let word_set: HashSet<String> = words.iter().cloned().collect();
        let mut dp: HashMap<String, i32> = HashMap::new();

        let mut max_chain = 1;
        for ws in words.into_iter() {
            let w: Vec<char> = ws.chars().collect();
            let mut cur_chain = 1;
            for i in 0..w.len() {
                let mut striped_one: String = w[0..i].iter().collect();
                striped_one.push_str(&w[i + 1..].iter().collect::<String>());

                if dp.contains_key(&striped_one) {
                    cur_chain = cur_chain.max(*dp.get(&striped_one).unwrap() + 1);
                }
            }
            dp.insert(ws, cur_chain);
            max_chain = max_chain.max(cur_chain);
        }

        max_chain
    }
}

fn main() {
    let mut input = vec![3, 1, 9];
    input.sort_by_key(|x| *x);
    println!("{:?}", input);
}
