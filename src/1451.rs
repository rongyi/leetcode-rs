#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn arrange_words(text: String) -> String {
        let mut words_with_index: Vec<(usize, String)> = text
            .split(' ')
            .map(|w| w.to_lowercase())
            .enumerate()
            .collect();

        words_with_index.sort_by(|a, b| a.1.len().cmp(&b.1.len()).then(a.0.cmp(&b.0)));

        let mut sorted: Vec<String> = words_with_index.into_iter().map(|s| s.1).collect();
        let mut first_cap: Vec<char> = sorted[0].clone().chars().collect();
        first_cap[0] = first_cap[0].to_ascii_uppercase();
        sorted[0] = first_cap.into_iter().collect();

        sorted.join(" ")
    }
}

fn main() {}
