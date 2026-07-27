struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .map(|w| w.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn main() {}
