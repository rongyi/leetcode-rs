#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut result = vec![0; n];

        if k == 0 {
            return result;
        }

        for i in 0..n {
            let mut sum = 0;

            if k > 0 {
                // Sum of k elements after the current element
                for j in 1..=k as usize {
                    sum += code[(i + j) % n];
                }
            } else {
                // Sum of |k| elements before the current element
                for j in 1..=(-k) as usize {
                    sum += code[(i + n - j) % n];
                }
            }

            result[i] = sum;
        }

        result
    }
}

fn main() {}
