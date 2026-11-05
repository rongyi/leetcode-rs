#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        // Split sentences into words
        let words1: Vec<&str> = sentence1.split_whitespace().collect();
        let words2: Vec<&str> = sentence2.split_whitespace().collect();

        // Handle edge cases: empty sentences or one sentence is the other
        if words1 == words2 {
            return true;
        }

        let (shorter, longer) = if words1.len() <= words2.len() {
            (&words1, &words2)
        } else {
            (&words2, &words1)
        };

        let n = shorter.len();
        let m = longer.len();

        if n == 0 {
            return true;
        }

        // Find longest prefix match
        let mut prefix_len = 0;
        while prefix_len < n && shorter[prefix_len] == longer[prefix_len] {
            prefix_len += 1;
        }

        // Find longest suffix match (starting from the end)
        let mut suffix_len = 0;
        while suffix_len < n - prefix_len
            && shorter[n - 1 - suffix_len] == longer[m - 1 - suffix_len]
        {
            suffix_len += 1;
        }

        // Check if prefix + suffix covers the entire shorter sentence
        prefix_len + suffix_len >= n
    }
}

fn main() {}
