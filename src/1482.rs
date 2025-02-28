#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        // Check if it's impossible to make m bouquets
        if (m as i64 * k as i64) > bloom_day.len() as i64 {
            return -1;
        }

        // Find minimum and maximum days
        let mut left = *bloom_day.iter().min().unwrap();
        let mut right = *bloom_day.iter().max().unwrap();

        // Binary search for the minimum days needed
        while left < right {
            let mid = left + (right - left) / 2;
            if Self::can_make_bouquets(&bloom_day, m, k, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    // Helper function to check if we can make m bouquets in 'days' days
    fn can_make_bouquets(bloom_day: &Vec<i32>, m: i32, k: i32, days: i32) -> bool {
        let mut bouquets = 0;
        let mut flowers = 0;

        for &day in bloom_day.iter() {
            if day <= days {
                flowers += 1;
                if flowers == k {
                    bouquets += 1;
                    flowers = 0;
                }
            } else {
                flowers = 0;
            }

            if bouquets >= m {
                return true;
            }
        }

        bouquets >= m
    }
}
fn main() {}
