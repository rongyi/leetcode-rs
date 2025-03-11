#![allow(dead_code)]

struct Solution;

impl Solution {
    // More efficient solution using HashMap
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        // Helper function to count triplets
        let count_triplets = |a: &Vec<i32>, b: &Vec<i32>| -> i32 {
            let mut count = 0;

            // Create a frequency map for squares of elements in a
            let mut square_freq: HashMap<i64, i32> = HashMap::new();
            for &num in a {
                let square = (num as i64).pow(2);
                *square_freq.entry(square).or_insert(0) += 1;
            }

            // Check pairs in b
            let n = b.len();
            for i in 0..n {
                for j in i + 1..n {
                    let product = (b[i] as i64) * (b[j] as i64);
                    if let Some(&freq) = square_freq.get(&product) {
                        count += freq;
                    }
                }
            }

            count
        };

        count_triplets(&nums1, &nums2) + count_triplets(&nums2, &nums1)
    }
}

fn main() {}
