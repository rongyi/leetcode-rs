
struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let line1 = "qwertyuiop";
        let line2 = "asdfghjkl";
        let line3 = "zxcvbnm";
        let mut char_line: HashMap<char, i32> = HashMap::new();
        for c in line1.chars() {
            char_line.insert(c, 0);
        }
        for c in line2.chars() {
            char_line.insert(c, 1);
        }

        for c in line3.chars() {
            char_line.insert(c, 2);
        }

        let mut ret = Vec::new();
        for w in words.into_iter() {
            let wl = w.to_lowercase();
            let mut line_set: HashSet<i32> = HashSet::new();
            for c in wl.chars() {
                line_set.insert(*char_line.get(&c).unwrap());
                if line_set.len() > 1 {
                    break;
                }
            }
            if line_set.len() == 1 {
                ret.push(w);
            }
        }

        ret
    }
}

fn main() {}
