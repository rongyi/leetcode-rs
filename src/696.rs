#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        // previous we use one_cont and zero count
        // but we dont need to differenciat the zero or one
        let mut prev_cnt = 0;
        let mut cur_cnt = 0;
        let mut ret = 0;
        let s: Vec<char> = s.chars().collect();
        for (i, &c) in s.iter().enumerate() {
            if i > 0 && s[i - 1] != c {
                ret += prev_cnt.min(cur_cnt);
                prev_cnt = cur_cnt;
                cur_cnt = 1;
            } else {
                // accumulation for current
                cur_cnt += 1;
            }
        }
        // final check
        ret += prev_cnt.min(cur_cnt);

        ret
    }
}

fn main() {
    let val = Solution::count_binary_substrings("10101".to_string());
    println!("{}", val);
}
