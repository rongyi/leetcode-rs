use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    /// LeetCode 819: Most Common Word
    ///
    /// Return the most frequent word in the paragraph that is not banned.
    /// Words are letter-only; separators are space and punctuation.
    /// Case-insensitive.
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut word_count: HashMap<String, i32> = HashMap::new();
        let banned: HashSet<String> = banned.into_iter().collect();
        let split: HashSet<char> = " !?',;.".chars().collect();

        let s: Vec<char> = paragraph.chars().collect();
        let sz = s.len();
        let mut i = 0;
        while i < sz {
            // Find the word [i..j): run of non-separator chars.
            let mut j = i;
            while j < sz && !split.contains(&s[j]) {
                j += 1;
            }

            let mut cur: String = s[i..j].iter().collect();
            cur = cur.to_lowercase();
            if !cur.is_empty() && !banned.contains(&cur) {
                *word_count.entry(cur).or_insert(0) += 1;
            }

            // Skip all separators to reach the next word.
            while j < sz && split.contains(&s[j]) {
                j += 1;
            }
            i = j;
        }

        // Find the most frequent word.
        let mut max_cnt = 0;
        let mut max_str = String::new();
        for (word, &cnt) in word_count.iter() {
            if cnt > max_cnt {
                max_cnt = cnt;
                max_str = word.clone();
            }
        }
        max_str
    }
}

fn main() {
    let tests = [
        (
            "Bob hit a ball, the hit BALL flew far after it was hit.",
            vec!["hit"],
            "ball",
        ),
        ("a, a, a, a, b,b,b,c, c", vec!["a"], "b"),
        ("Lion, Lion, cat, cat!", vec!["lion"], "cat"),
    ];

    for (paragraph, banned, expected) in &tests {
        let result = Solution::most_common_word(
            paragraph.to_string(),
            banned.iter().map(|s| s.to_string()).collect(),
        );
        println!(
            "{} paragraph=\"{}\" → \"{}\" (expected \"{}\")",
            if result == *expected { "✓" } else { "✗" },
            paragraph,
            result,
            expected
        );
    }
}
