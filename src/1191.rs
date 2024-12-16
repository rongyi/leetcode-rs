#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn k_concatenation_max_sum(mut arr: Vec<i32>, k: i32) -> i32 {
        let m = 1e9 as i64 + 7;
        let max_sum = Self::kadanne(&arr);
        if k == 1 {
            return max_sum as i32 % m as i32;
        }
        let total_sum: i64 = arr.iter().map(|&x| x as i64).sum();
        arr.extend(arr.clone());
        let max_sum = Self::kadanne(&arr);
        let ret = if max_sum > 0 {
            max_sum + (total_sum * (k as i64 - 2)).max(0)
        } else {
            max_sum
        };

        (ret % m) as i32
    }

    fn kadanne(nums: &[i32]) -> i64 {
        let mut max_sum = 0;
        let mut sum = 0;
        for &num in nums.iter() {
            sum = (sum + num as i64).max(0);
            max_sum = max_sum.max(sum);
        }
        max_sum
    }
}

fn main() {}
