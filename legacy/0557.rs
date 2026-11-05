struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ret: Vec<String> = Vec::new();
        for chunk in s.split_whitespace() {
            let cur = chunk.chars().rev().collect();
            ret.push(cur);
        }

        ret.join(" ")
    }
}

fn main() {}
