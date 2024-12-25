#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        Self::at_most_k(&nums, k) - Self::at_most_k(&nums, k - 1)
    }

    fn at_most_k(nums: &Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let mut acc = 0;
        let mut i = 0;
        for (j, &num) in nums.iter().enumerate() {
            if num % 2 == 1 {
                acc += 1;
            }
            while acc > k {
                if nums[i] % 2 == 1 {
                    acc -= 1;
                }
                i += 1;
            }
            ret += j - i + 1;
        }

        ret as i32
    }
}

fn main() {}
