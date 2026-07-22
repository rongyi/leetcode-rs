struct Solution;

impl Solution {
    /// LeetCode 524: Longest Word in Dictionary through Deleting
    ///
    /// Return the longest word from `dictionary` that is a subsequence
    /// of `s`. Tiebreak: smallest lexicographical order.
    ///
    /// # Approach: Sort + two-pointer subsequence check
    ///
    /// Sort by length descending, then lexicographically ascending.
    /// Return the first word that is a subsequence of s.
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut dict = dictionary;
        // Longest first; if equal, lexicographically smallest first.
        dict.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));

        for word in &dict {
            if Self::is_subsequence(word, &s) {
                return word.clone();
            }
        }

        String::new()
    }

    /// Two-pointer: returns true if `word` is a subsequence of `s`.
    fn is_subsequence(word: &str, s: &str) -> bool {
        let (mut i, mut j) = (0, 0);
        let (w, s_bytes) = (word.as_bytes(), s.as_bytes());
        while i < w.len() && j < s_bytes.len() {
            if w[i] == s_bytes[j] {
                i += 1;
            }
            j += 1;
        }
        i == w.len()
    }
}

fn main() {
    let tests = [
        ("abpcplea", vec!["ale", "apple", "monkey", "plea"], "apple"),
        ("abpcplea", vec!["a", "b", "c"], "a"),
        ("a", vec!["b", "c"], ""),
    ];

    for (s, dict, expected) in &tests {
        let result = Solution::find_longest_word(
            s.to_string(),
            dict.iter().map(|w| w.to_string()).collect(),
        );
        println!(
            "{} s=\"{}\" dict={:?} → \"{}\" (expected \"{}\")",
            if result == *expected { "✓" } else { "✗" },
            s,
            dict,
            result,
            expected
        );
    }
}
