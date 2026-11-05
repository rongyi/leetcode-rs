struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_groups_for_valid_assignment(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        let frequencies: Vec<i32> = counts.values().cloned().collect();
        let min_f = *frequencies.iter().min().unwrap();

        // We try the largest possible group size k first to minimize total groups.
        // A group can be size k or k + 1.
        for k in (1..=min_f).rev() {
            if let Some(total_groups) = Self::calculate_groups(&frequencies, k) {
                return total_groups;
            }
        }

        0 // Should not reach here given constraints
    }

    fn calculate_groups(frequencies: &[i32], k: i32) -> Option<i32> {
        let mut total = 0;
        for &f in frequencies {
            // How many groups of (k + 1) can we fit?
            let num_groups = f / (k + 1);
            let remainder = f % (k + 1);

            if remainder == 0 {
                total += num_groups;
            } else {
                // If there's a remainder, we need one more group.
                // Can we distribute the (k + 1 - remainder) shortfall
                // by taking 1 away from existing (k + 1) groups?
                // This is possible if we have enough groups to absorb the difference.
                let needed = k - remainder;
                if needed <= num_groups {
                    total += num_groups + 1;
                } else {
                    return None; // This k doesn't work for this frequency
                }
            }
        }
        Some(total)
    }
}

fn main() {}
