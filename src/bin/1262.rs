#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut m1 = 20000;
        let mut m2 = 20000;
        for &num in nums.iter() {
            ret += num;
            if num % 3 == 1 {
                m2 = m2.min(m1 + num);
                m1 = m1.min(num);
            } else if num % 3 == 2 {
                m1 = m1.min(m2 + num);
                m2 = m2.min(num);
            }
        }
        if ret % 3 == 0 {
            return ret;
        }
        if ret % 3 == 1 {
            return ret - m1;
        }
        ret - m2
    }
}

fn main() {}
