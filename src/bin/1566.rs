#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        if arr.len() < (m as usize) * (k as usize) {
            return false;
        }

        let m = m as usize;
        let k = k as usize;

        for i in 0..=arr.len() - m * k {
            let mut j = 0;
            while j < m * k {
                if arr[i + j] != arr[i + (j % m)] {
                    break;
                }
                j += 1;
            }
            if j == m * k {
                return true;
            }
        }

        false
    }
}

fn main() {}
