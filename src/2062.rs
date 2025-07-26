struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let word: Vec<char> = word.chars().collect();

        let sz = word.len();
        if sz < 5 {
            return 0;
        }
        fn is_vowel(c: char) -> bool {
            match c {
                'a' | 'o' | 'e' | 'i' | 'u' => true,
                _ => false,
            }
        }
        let mut sum = 0;
        for i in 0..=sz - 5 {
            if !is_vowel(word[i]) {
                continue;
            }
            let mut uniq: HashSet<char> = HashSet::new();
            uniq.insert(word[i]);
            let mut j = i + 1;
            while j < sz && is_vowel(word[j]) {
                uniq.insert(word[j]);
                if uniq.len() == 5 {
                    sum += 1;
                }
                j += 1;
            }
        }

        sum
    }
}

fn main() {}
