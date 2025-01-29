#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let target = k * threshold;
        let k = k as usize;
        let mut win_sum: i32 = arr[..k].iter().sum();
        let mut total = 0;

        if win_sum >= target {
            total += 1;
        }
        for i in k..arr.len() {
            win_sum += arr[i];
            win_sum -= arr[i - k];
            if win_sum >= target {
                total += 1;
            }
        }

        total
    }
}

fn main() {}
