#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let d_start = Self::digit_count(low);
        let d_end = Self::digit_count(high);
        let mut ret = vec![];
        for d in d_start..=d_end {
            for start in 1..=(10 - d) {
                let mut val = 0;
                for i in 0..d {
                    val = val * 10 + (start + i);
                }

                if val < low {
                    continue;
                }
                if val > high {
                    break;
                }

                ret.push(val);
            }
        }
        ret
    }
    fn digit_count(mut n: i32) -> i32 {
        let mut d = 0;
        while n != 0 {
            d += 1;
            n /= 10;
        }
        d
    }
}

fn main() {
    Solution::sequential_digits(100, 300);
}
