struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut cnt: BTreeMap<char, i32> = BTreeMap::new();
        for c in s.chars() {
            *cnt.entry(c).or_default() += 1;
        }
        let mut ret = String::new();

        // lexicographically largest
        while !cnt.is_empty() {
            let (c, mut total) = cnt.pop_last().unwrap();
            let mut used = 0;
            while total > 0 {
                used += 1;

                if used > repeat_limit {
                    if cnt.is_empty() {
                        return ret;
                    }
                    if let Some((&gap, v)) = cnt.iter_mut().rev().next() {
                        *v -= 1;
                        if *v == 0 {
                            cnt.pop_last();
                        }
                        ret.push(gap);
                        used = 1;
                    }
                }

                ret.push(c);
                total -= 1;
            }
        }

        ret
    }
}

fn main() {}
