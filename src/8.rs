struct Solution;

use std::i32;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.as_bytes();
        let sz = s.len();

        let mut i = 0;
        // ignore leading whitespaces
        while i < sz && s[i] == b' ' {
            i += 1;
        }
        if i == sz {
            return 0;
        }
        let mut is_neg = false;
        // sign
        if s[i] == b'-' {
            is_neg = true;
            i += 1;
        } else if s[i] == b'+' {
            i += 1;
        }
        let mut acc = 0;

        while i < sz {
            if s[i] > b'9' || s[i] < b'0' {
                break;
            }
            let mut cur = (s[i] - b'0') as i32;

            if is_neg {
                cur = -cur;
            }

            if acc > i32::MAX / 10 || (acc == i32::MAX / 10 && cur > i32::MAX % 10) {
                return i32::MAX;
            }
            if acc < i32::MIN / 10 || (acc == i32::MIN / 10 && cur < i32::MIN % 10) {
                return i32::MIN;
            }

            acc = acc * 10 + cur;

            i += 1;
        }

        acc
    }
}

fn main() {}
