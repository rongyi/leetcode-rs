
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<String> = s.split_whitespace().map(String::from).collect();
        let mut bind1: HashMap<char, String> = HashMap::new();
        let mut bind2: HashMap<String, char> = HashMap::new();
        if words.len() != pattern.len() {
            return false;
        }
        for (i, c) in pattern.chars().enumerate() {
            match (bind1.get(&c), bind2.get(&words[i])) {
                (Some(a), Some(b)) => {
                    if *b != c || *a != words[i] {
                        return false;
                    }
                }
                (None, Some(_)) => return false,
                (Some(_), None) => return false,
                (None, None) => {
                    bind1.insert(c, words[i].clone());
                    bind2.insert(words[i].clone(), c);
                }
            }
        }

        true
    }
}

fn main() {
    unimplemented!();
}
