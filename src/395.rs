struct Solution;

impl Solution {
    /// LeetCode 395: Longest Substring with At Least K Repeating Characters
    ///
    /// Divide-and-conquer: a character with frequency < k in the current
    /// segment can never be part of a valid answer. Split on it, recurse.
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::helper(&s.chars().collect::<Vec<_>>(), k)
    }

    fn helper(s: &[char], k: i32) -> i32 {
        // Count frequencies in this segment.
        let mut freq = [0i32; 26];
        for &c in s {
            freq[c as usize - 'a' as usize] += 1;
        }

        // Find a split character (appears < k times overall in this segment).
        let split_char = s.iter().find(|&&c| freq[c as usize - 'a' as usize] < k);

        match split_char {
            // Every character appears >= k → this whole segment is valid.
            None => s.len() as i32,
            // Split at each occurrence of the bad character, recurse on each part.
            Some(&bad) => {
                let mut max_len = 0;
                let mut start = 0;
                for (end, &c) in s.iter().enumerate() {
                    if c == bad {
                        if start < end {
                            max_len = max_len.max(Self::helper(&s[start..end], k));
                        }
                        start = end + 1;
                    }
                }
                // Trailing segment after the last split.
                if start < s.len() {
                    max_len = max_len.max(Self::helper(&s[start..], k));
                }
                max_len
            }
        }
    }
}

fn main() {
    let tests = [
        ("aaabb", 3, 3),
        ("ababbc", 2, 5),
        ("ababacb", 3, 0),
        ("aaaaaaaaabbbbbbbbbbbb", 3, 21),
        ("a", 1, 1),
    ];

    for (s, k, expected) in &tests {
        let result = Solution::longest_substring(s.to_string(), *k);
        println!(
            "{} s=\"{}\" k={} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            s,
            k,
            result,
            expected
        );
    }
}
