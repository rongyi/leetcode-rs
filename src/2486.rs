struct Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;
        let mut j = 0;
        let sz1 = s.len();
        let sz2 = t.len();
        while i < sz1 && j < sz2 {
            if s[i] == t[j] {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }

        (sz2 - j) as _
    }
}

fn main() {
    Solution::append_characters("coaching".to_string(), "coding".to_string());
}
