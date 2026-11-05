#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_probability(balls: Vec<i32>) -> f64 {
        let sum: i32 = balls.iter().sum();
        let mut left = vec![0; balls.len()];
        let mut right = vec![0; balls.len()];

        Self::dfs(&balls, &mut left, &mut right, sum, 0, 0, 0) / Self::perm(&balls)
    }

    fn perm(nums: &Vec<i32>) -> f64 {
        let mut ret = 1.0;
        let mut j = 1.0;
        for &val in nums.iter() {
            for k in 1..=val {
                // in case of overflow
                ret = ret * j / k as f64;
                j += 1.0;
            }
        }

        ret
    }
    fn dfs(
        balls: &Vec<i32>,
        left: &mut Vec<i32>,
        right: &mut Vec<i32>,
        sum: i32,
        cur_idx: usize,
        left_total: i32,
        right_total: i32,
    ) -> f64 {
        if left_total > sum / 2 || right_total > sum / 2 {
            return 0.0;
        }
        if cur_idx == balls.len() {
            let left_colors = left.iter().filter(|&&x| x > 0).count();
            let right_colors = right.iter().filter(|&&x| x > 0).count();
            if left_colors != right_colors {
                return 0.0;
            }

            return Self::perm(left) * Self::perm(right);
        }

        let mut ret = 0.0;
        for i in 0..=balls[cur_idx] {
            left[cur_idx] = i;
            right[cur_idx] = balls[cur_idx] - i;
            ret += Self::dfs(
                balls,
                left,
                right,
                sum,
                cur_idx + 1,
                left_total + left[cur_idx],
                right_total + right[cur_idx],
            );
        }

        ret
    }
}

fn main() {}
