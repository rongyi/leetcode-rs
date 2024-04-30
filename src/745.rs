#![allow(dead_code)]

use std::collections::HashMap;

struct WordFilter {
    data: HashMap<String, i32>,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut data = HashMap::new();
        for (i, w) in words.into_iter().enumerate() {
            let w: Vec<_> = w.chars().collect();
            let mut p = String::new();
            let sz = w.len();
            for j in 0..sz {
                p.push(w[j]);
                let mut s = String::new();
                for k in (0..sz).rev() {
                    s.insert(0, w[k]);
                    let key = p.clone() + "|" + &s.clone();
                    data.insert(key, i as i32 + 1);
                }
            }
        }

        Self { data }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let key = pref + "|" + &suff;
        self.data.get(&key).unwrap_or(&0) - 1
    }
}

fn main() {}
