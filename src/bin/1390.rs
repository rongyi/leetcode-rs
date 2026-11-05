#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for num in nums.into_iter() {
            sum += Self::divisor_cnt(num);
        }

        sum
    }
    fn divisor_cnt(num: i32) -> i32 {
        let mut cnt = 0;
        let mut sum = 0;
        for i in 1.. {
            if i * i > num {
                break;
            }
            if num % i == 0 {
                if num / i == i {
                    sum += i;
                    cnt += 1;
                } else {
                    cnt += 2;
                    sum += i + num / i;
                }
            }
        }
        if cnt == 4 {
            sum
        } else {
            0
        }
    }
}

fn main() {
    let input = vec![21, 4, 7];
    let a = Solution::sum_four_divisors(input);
    println!("{}", a);
}
