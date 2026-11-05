#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
        let mut count = std::collections::HashMap::new();
        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        let mut counts: Vec<i32> = count.values().copied().collect();
        let mut quantity = quantity;
        quantity.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order

        fn backtrack(counts: &mut Vec<i32>, quantity: &Vec<i32>, index: usize) -> bool {
            if index == quantity.len() {
                return true;
            }

            for i in 0..counts.len() {
                if counts[i] >= quantity[index] {
                    counts[i] -= quantity[index];
                    if backtrack(counts, quantity, index + 1) {
                        return true;
                    }
                    counts[i] += quantity[index];
                }
            }

            false
        }

        backtrack(&mut counts, &quantity, 0)
    }
}

fn main() {}
