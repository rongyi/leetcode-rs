struct Solution;

impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let s = text.as_bytes();
        let sz = s.len();
        let mut left = 0;
        let mut right = sz;
        let mut ret = 0;
        while left < right {
            let mut found = false;
            let max_len = right - left;
            for len in 1..=max_len / 2 {
                if left + len <= right - len {
                    let prefix = &s[left..left + len];
                    let suffix = &s[right - len..right];
                    if prefix == suffix {
                        ret += 2;
                        left += len;
                        right -= len;
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                ret += 1;
                break;
            }
        }

        ret
    }
}

fn main() {}
