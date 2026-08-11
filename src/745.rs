struct Solution;

use std::collections::HashMap;

struct WordFilter {
    data: HashMap<String, i32>,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut data = HashMap::new();
        for (idx, w) in words.into_iter().enumerate() {
            // let w: Vec<_> = w.chars().collect();
            let sz = w.len();

            // Generate all possible prefix-suffix combinations
            for i in 0..=sz {
                for j in 0..=sz {
                    let prefix = &w[0..i];
                    let suffix = &w[sz - j..];
                    let key = format!("{}|{}", prefix, suffix);
                    data.insert(key, idx as i32);
                }
            }
            // for j in 0..sz {
            //     p.push(w[j]);
            //     let mut s = String::new();
            //     for k in (0..sz).rev() {
            //         s.insert(0, w[k]);
            //         let key = p.clone() + "|" + &s.clone();
            //         data.insert(key, i as i32 + 1);
            //     }
            // }
        }

        Self { data }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let key = pref + "|" + &suff;
        *self.data.get(&key).unwrap_or(&-1)
    }
}

fn main() {}
