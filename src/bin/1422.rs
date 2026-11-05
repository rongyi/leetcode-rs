#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        // including itself
        let mut right_side_ones: Vec<i32> = vec![0; sz];
        let mut acc = 0;
        for i in (0..sz).rev() {
            if s[i] == '1' {
                acc += 1;
            }
            right_side_ones[i] = acc;
        }
        let mut left_zeros_acc = 0;
        let mut ret = 0;
        for i in 0..sz - 1 {
            if s[i] == '0' {
                left_zeros_acc += 1;
            }
            // try split here
            ret = ret.max(left_zeros_acc + right_side_ones[i + 1]);
        }
        ret
    }
}

fn main() {}
