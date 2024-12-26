#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        let mut gcd = nums[0];
        for &num in nums.iter() {
            gcd = Self::gcd(gcd, num);
        }

        gcd == 1
    }
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let tmp = a % b;
            a = b;
            b = tmp;
        }
        a
    }
}

fn main() {}
