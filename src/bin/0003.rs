struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let sz = s.len();
        let mut i = 0;
        let mut max_len = 0;
        let mut cnt: HashMap<u8, usize> = HashMap::new();
        for j in 0..sz {
            *cnt.entry(s[j]).or_default() += 1;
            while i < j && cnt.len() != j - i + 1 {
                // cnt.entry(s[i]).and_modify(|x| *x -= 1);
                if let Some(v) = cnt.get_mut(&s[i]) {
                    *v -= 1;
                    if *v == 0 {
                        cnt.remove(&s[i]);
                    }
                }
                i += 1;
            }

            max_len = max_len.max(j - i + 1);
        }

        max_len as _
    }
}

fn main() {}
