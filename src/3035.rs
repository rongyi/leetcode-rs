struct Solution;

impl Solution {
    pub fn max_palindromes_after_operations(words: Vec<String>) -> i32 {
        let mut char_counts = [0; 26];
        let mut word_lengths = Vec::new();

        // 1. Fill the global pool and record lengths
        for word in &words {
            word_lengths.push(word.len());
            for &b in word.as_bytes() {
                char_counts[(b - b'a') as usize] += 1;
            }
        }

        // 2. Count total available pairs
        let mut total_pairs = 0;
        for count in char_counts {
            total_pairs += count / 2;
        }

        // 3. Sort lengths to prioritize shortest words (Greedy)
        word_lengths.sort_unstable();

        let mut max_palindromes = 0;
        for length in word_lengths {
            // A word of length L needs L/2 pairs to be a palindrome
            let pairs_needed = length / 2;

            if total_pairs >= pairs_needed {
                total_pairs -= pairs_needed;
                max_palindromes += 1;
            } else {
                // Not enough pairs to satisfy this word
                break;
            }
        }

        max_palindromes
    }
}

fn main() {}
