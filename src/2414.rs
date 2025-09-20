struct Solution;

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let sz = s.len();
        let mut i = 1;
        let mut max_len = 1;

        let mut cur_len = 1;
        while i < sz {
            let mut j = i;
            while j < sz && s[j] == s[j - 1] + 1 {
                j += 1;
                cur_len += 1;
            }
            // no change
            if j == i {
                i += 1;
                continue;
            }

            // calculate
            max_len = max_len.max(cur_len);
            i = j;
            cur_len = 1;
        }

        max_len as _
    }
}

fn main() {
    Solution::longest_continuous_substring("abacaba".to_string());
}
