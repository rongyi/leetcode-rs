struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut out = Vec::new();
        for c in s.chars() {
            if c == 'B' {
                if !out.is_empty() && *out.last().unwrap() == 'A' {
                    out.pop();
                } else {
                    out.push('B');
                }
            } else if c == 'D' {
                if !out.is_empty() && *out.last().unwrap() == 'C' {
                    out.pop();
                } else {
                    out.push('D');
                }
            } else {
                out.push(c);
            }
        }
        out.len() as _
    }
}

fn main() {
    let input = "ABFCACDB".to_string();
    Solution::min_length(input);
}
