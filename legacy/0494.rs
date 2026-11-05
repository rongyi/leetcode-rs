struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut cnt = 0;
        Self::dfs(&nums, target as i64, 0, &mut cnt);

        cnt
    }

    fn dfs(nums: &[i32], target: i64, idx: usize, cnt: &mut i32) {
        if idx == nums.len() {
            if target == 0 {
                *cnt += 1;
            }
            return;
        }

        Self::dfs(nums, target - nums[idx] as i64, idx + 1, cnt);
        Self::dfs(nums, target + nums[idx] as i64, idx + 1, cnt);
    }
}

fn main() {}
