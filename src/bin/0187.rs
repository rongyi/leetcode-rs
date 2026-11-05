
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let sz = s.len();
        let mut visited: HashMap<&[u8], usize> = HashMap::new();
        if sz < 10 {
            return vec![];
        }
        let s = s.as_bytes();
        for i in 0..=sz - 10 {
            let cur = &s[i..i + 10];
            *visited.entry(cur).or_default() += 1;
        }

        visited
            .iter()
            .filter_map(|v| {
                if *v.1 > 1 {
                    Some(String::from_utf8_lossy(v.0).to_string())
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() {}
