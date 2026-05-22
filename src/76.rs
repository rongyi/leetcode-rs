struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut min_len = s.len() + 1;
        let mut min_start = 0;

        let mut expect: HashMap<u8, i32> = HashMap::new();
        for &b in t.iter() {
            *expect.entry(b).or_default() += 1;
        }
        let mut i = 0;
        let mut j = 0;
        let sz = s.len();
        // we need those match to t in s substr
        let mut need_match = t.len();

        while j < sz {
            if let Some(v) = expect.get_mut(&s[j]) {
                // consume one real match
                if *v > 0 {
                    need_match -= 1;
                }
                *v -= 1;
            }
            j += 1;

            while need_match == 0 {
                // find a valid match
                if j - i < min_len {
                    min_len = j - i;
                    min_start = i;
                }
                // put back those
                if let Some(v) = expect.get_mut(&s[i]) {
                    // when v < 0 , it means we have way more this char after, dont panic
                    if *v >= 0 {
                        need_match += 1;
                    }
                    *v += 1;
                }

                i += 1;
            }
        }
        if min_len > s.len() {
            return "".to_string();
        }

        String::from_utf8_lossy(&s[min_start..min_start + min_len]).to_string()
    }
}

fn main() {}
