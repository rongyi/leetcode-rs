struct Solution;

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        Self::backtrack(num.as_bytes(), None, None, 0, false)
    }

    fn backtrack(
        s: &[u8],
        num1: Option<i32>,
        num2: Option<i32>,
        start: usize,
        prev_valid: bool,
    ) -> bool {
        if start == s.len() {
            return num1.is_some() && num2.is_some() && prev_valid;
        }

        let mut acc = 0;
        for i in start..s.len() {
            if i > start && s[start] == b'0' {
                break;
            }
            acc = acc * 10 + (s[i] as u8 - b'0' as u8) as i32;
            match (num1, num2) {
                (Some(v1), Some(v2)) => {
                    if v1 + v2 == acc && Self::backtrack(s, Some(v2), Some(acc), i + 1, true) {
                        return true;
                    }
                }
                (None, None) => {
                    if Self::backtrack(s, Some(acc), None, i + 1, prev_valid) {
                        return true;
                    }
                }
                (Some(v1), None) => {
                    if Self::backtrack(s, Some(v1), Some(acc), i + 1, prev_valid) {
                        return true;
                    }
                }
                _ => unreachable!(),
            }
        }

        false
    }
}

fn main() {}
