struct Solution;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let mut max_len = 0;
        let mut cnt = vec![0; 26];
        let s: Vec<char> = s.chars().collect();
        for &c in s.iter() {
            cnt[c as usize - 'a' as usize] += 1;
        }
        let mut start = 0;
        let mut end = 0;
        // split at char which cnt < k
        while end < s.len() {
            if cnt[s[end] as usize - 'a' as usize] < k {
                max_len = max_len.max(Self::longest_substring(s[start..end].iter().collect(), k));
                start = end + 1;
            }
            end += 1;
        }

        if start == 0 {
            s.len() as i32
        } else {
            max_len.max(Self::longest_substring(s[start..].iter().collect(), k))
        }
    }
}

fn main() {}
