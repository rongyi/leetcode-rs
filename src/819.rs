#![allow(dead_code)]

struct Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut word_count: HashMap<String, i32> = HashMap::new();
        let banned: HashSet<String> = banned.into_iter().collect();
        let split: HashSet<char> = " !?',;.".chars().collect();

        let mut s: Vec<char> = paragraph.chars().collect();
        s.push(' ');
        let sz = paragraph.len();
        let mut i = 0;
        while i < sz {
            let mut j = i;
            while j < sz && !split.contains(&s[j]) {
                j += 1;
            }
            let mut cur: String = s[i..j].to_vec().into_iter().collect();
            cur = cur.to_lowercase();
            // println!("cur: {}", cur);
            if !banned.contains(&cur) {
                *word_count.entry(cur).or_insert(0) += 1;
            }

            // skip more
            while j < sz && split.contains(&s[j]) {
                j += 1;
            }

            i = j;
        }

        let mut max_cnt = 0;
        let mut max_str = "".to_string();
        for (k, &v) in word_count.iter() {
            if v > max_cnt {
                max_cnt = v;
                max_str = k.clone();
            }
        }

        max_str
    }
}

fn main() {
    let input = vec!["bob".to_string(), "hit".to_string()];
    let val = Solution::most_common_word("Bob. hIt, baLl".to_string(), input);
    println!("{}", val);
}
