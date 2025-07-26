struct Solution;

impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let mut sum = 0;
        let word: Vec<char> = word.chars().collect();
        fn is_vowel(c: char) -> bool {
            match c {
                'a' | 'o' | 'e' | 'i' | 'u' => true,
                _ => false,
            }
        }
        for (i, &c) in word.iter().enumerate() {
            if is_vowel(c) {
                // end with cur substrings
                sum += (i + 1) as i64 * (word.len() - i) as i64;
            }
        }

        sum
    }
}

fn main() {}
