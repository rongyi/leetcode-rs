struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut group: HashMap<char, i32> = HashMap::new();

        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        for c in t.iter() {
            *group.entry(*c).or_insert(0) += 1;
        }

        let mut i = 0;
        let mut j = 0;
        let mut min_len = s.len() + 1;
        let mut min_start = 0;
        let mut need_match = t.len();

        while j < s.len() {
            // if we match a char in dict, we decrese it
            if let Some(val) = group.get_mut(&s[j]) {
                if *val > 0 {
                    need_match -= 1;
                }
                *val -= 1;
            }
            j += 1;

            while need_match == 0 {
                if j - i < min_len {
                    min_len = j - i;
                    min_start = i;
                }
                if let Some(val) = group.get_mut(&s[i]) {
                    if *val >= 0 {
                        // add it back, means we need more match
                        need_match += 1;
                    }
                    *val += 1;
                }
                i += 1;
            }
        }

        if min_len > s.len() {
            return String::new();
        }

        s[min_start..min_start + min_len].iter().collect()
    }
}

fn main() {
    unimplemented!();
}
