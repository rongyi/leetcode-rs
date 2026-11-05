struct Solution;
use std::collections::HashMap;


impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let expect = Self::normalize(&pattern);

        words
            .into_iter()
            .filter(|w| Self::normalize(w) == expect)
            .collect()
    }
    fn normalize(s: &str) -> Vec<usize> {
        let mut char_to_index = HashMap::new();
        let mut ret = Vec::with_capacity(s.len());
        for c in s.chars() {
            let idx = char_to_index.len();
            let v = char_to_index.entry(c).or_insert(idx);
            ret.push(*v);
        }

        ret
    }
}

fn main() {
    let val = Solution::normalize("helllo");
    println!("{:?}", val);
}
