pub struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        let total = nums.iter().sum::<i32>();
        let n = nums.len();
        let k = n / 2;
        let mut min_diff = i32::MAX;

        let mut left = vec![vec![]; k + 1];
        left[0].push(0);

        for &num in &nums[..k] {
            for i in (1..=k).rev() {
                for &sum in &left[i - 1].clone() {
                    left[i].push(sum + num);
                }
            }
        }

        for sums in &mut left {
            sums.sort();
        }

        let mut right = vec![vec![]; k + 1];
        right[0].push(0);

        for &num in &nums[k..] {
            for i in (1..=k).rev() {
                for &sum in &right[i - 1].clone() {
                    right[i].push(sum + num);
                }
            }
        }

        for sums in &mut right {
            sums.sort();
        }

        for i in 0..=k {
            let j = k - i;
            let left_sums = &left[i];
            let right_sums = &right[j];

            for &sum1 in left_sums {
                let target = (total - 2 * sum1) / 2;
                let idx = right_sums.binary_search(&target).unwrap_or_else(|x| x);

                if idx < right_sums.len() {
                    let sum2 = right_sums[idx];
                    let diff = (total - 2 * (sum1 + sum2)).abs();
                    min_diff = min_diff.min(diff);
                }

                if idx > 0 {
                    let sum2 = right_sums[idx - 1];
                    let diff = (total - 2 * (sum1 + sum2)).abs();
                    min_diff = min_diff.min(diff);
                }
            }
        }
        min_diff
    }
}

fn main() {}
