struct Solution;

impl Solution {
    pub fn sum_distance(mut nums: Vec<i32>, s: String, d: i32) -> i32 {
        let mut nums: Vec<i64> = nums.into_iter().map(|v| v as i64).collect();
        for (i, c) in s.chars().enumerate() {
            if c == 'L' {
                nums[i] -= d as i64;
            } else {
                nums[i] += d as i64;
            }
        }
        nums.sort_unstable();
        let mut prefix = 0i64;
        let mut sum = 0;
        let m = 1e9 as i64 + 7;

        for (i, v) in nums.into_iter().enumerate() {
            sum = (sum + i as i64 * v as i64 - prefix) % m;
            prefix += v as i64;
        }

        sum as _
    }
}

fn main() {}
