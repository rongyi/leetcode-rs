
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        fn count_chars(word: &str) -> HashMap<char, i32> {
            word.chars().fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            })
        }

        let max_freq: HashMap<char, i32> =
            words2.iter().fold(HashMap::new(), |mut max_map, word| {
                let word_freq = count_chars(word);
                for (c, &cnt) in word_freq.iter() {
                    let e = max_map.entry(*c).or_insert(0);
                    *e = (*e).max(cnt);
                }

                max_map
            });

        words1
            .into_iter()
            .filter(|w| {
                let word_freq = count_chars(w);
                max_freq
                    .iter()
                    .all(|(&c, &cnt)| word_freq.get(&c).unwrap_or(&0) >= &cnt)
            })
            .collect()
    }
}

fn main() {}
