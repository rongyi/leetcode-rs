#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut freq = std::collections::HashMap::new();
        for num in arr {
            *freq.entry(num).or_insert(0) += 1;
        }
        let mut freq_vec: Vec<i32> = freq.values().copied().collect();
        freq_vec.sort_unstable();
        let mut k = k;
        let mut unique = freq_vec.len() as i32;
        for &f in &freq_vec {
            if k >= f {
                k -= f;
                unique -= 1;
            } else {
                break;
            }
        }
        unique
    }
}

fn main() {}
