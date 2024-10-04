struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let wordset: HashSet<String> = wordlist.iter().cloned().collect();
        let mut cap_map: HashMap<String, String> = HashMap::new();
        let mut vowel_map: HashMap<String, String> = HashMap::new();

        for word in wordlist.iter() {
            let lower = word.to_lowercase();
            let devowel = Self::remove_vowels(&lower);

            cap_map.entry(lower).or_insert(word.clone());
            vowel_map.entry(devowel).or_insert(word.clone());
        }

        queries
            .into_iter()
            .map(|query| {
                if wordset.contains(&query) {
                    query
                } else {
                    let lower = query.to_lowercase();
                    let devowel = Self::remove_vowels(&lower);

                    cap_map
                        .get(&lower)
                        .or_else(|| vowel_map.get(&devowel))
                        .cloned()
                        .unwrap_or("".to_string())
                }
            })
            .collect()
    }

    fn remove_vowels(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' => '_',
                _ => c,
            })
            .collect()
    }
}

fn main() {}
