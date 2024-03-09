struct Solution;

impl Solution {
    pub fn makesquare(nums: Vec<i32>) -> bool {
        let sz = nums.len();
        if sz < 4 {
            return false;
        }
        let sum: i32 = nums.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        let side = sum / 4;
        let mut nums = nums;
        let mut ret = vec![0; 4];
        nums.sort_by(|a, b| b.cmp(a));

        Self::dfs(&nums, &mut ret, 0, side)
    }

    fn dfs(nums: &[i32], ret: &mut Vec<i32>, index: usize, target: i32) -> bool {
        if index == nums.len() {
            return ret[0] == target && ret[1] == target && ret[2] == target;
        }

        // try put current stick in four edge and test
        for i in 0..4 {
            if ret[i] + nums[index] > target {
                continue;
            }
            ret[i] += nums[index];
            if Self::dfs(nums, ret, index + 1, target) {
                return true;
            }
            ret[i] -= nums[index];
        }

        false
    }
}

fn main() {}
