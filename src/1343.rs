#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let sz = arr.len();
        let mut prefix = vec![0; sz + 1];
        for (i, &num) in arr.iter().enumerate() {
            prefix[i + 1] = prefix[i] + num as i64;
        }
        let k = k as usize;
        let threshold = threshold as i64;

        let mut total = 0;
        for i in 0..sz {
            let mut sum = prefix[i + 1];
            if i >= k {
                sum -= prefix[i - k + 1];
            }

            if i >= k - 1 {
                if sum / k as i64 >= threshold {
                    total += 1;
                }
            }
        }

        total
    }
}

fn main() {}
