#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let s: Vec<char> = text.chars().collect();
        let sz = s.len();
        let mut freq = HashMap::new();
        for &c in s.iter() {
            *freq.entry(c).or_insert(0) += 1;
        }
        let mut ret = 0;

        for i in 0..sz {
            if i > 0 && s[i] == s[i - 1] {
                continue;
            }

            let c = s[i];
            let mut cur_cnt = 0;
            let mut j = i;
            let mut gap_size = 0;

            while j < sz && (s[j] == c || gap_size == 0) {
                if s[j] == c {
                    cur_cnt += 1;
                } else {
                    gap_size = 1;
                    let mut k = j + 1;
                    while k < sz && s[k] == c {
                        cur_cnt += 1;
                        k += 1;
                    }
                    break;
                }
                j += 1;
            }
            if let Some(&total_freq) = freq.get(&c) {
                if cur_cnt < total_freq {
                    cur_cnt += 1;
                }
            }
            ret = ret.max(cur_cnt);
        }

        ret
    }
}

fn main() {}
