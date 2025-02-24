#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let s: Vec<char> = s.chars().collect();
        let k = k as usize;
        let sz = 1 << k;
        let mut found = vec![false; sz];
        let mut cur_num = 0;
        let mask = sz - 1;
        let mut acc_cnt = 0;

        for (i, &c) in s.iter().enumerate() {
            let val = if c == '0' { 0 } else { 1 };
            cur_num = mask & ((cur_num << 1) + val);
            if i >= k - 1 {
                if !found[cur_num] {
                    acc_cnt += 1;
                }
                found[cur_num] = true;
            }
        }

        acc_cnt == found.len()
    }
}

fn main() {}
