struct Solution;

impl Solution {
    pub fn num_squareful_perms(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ret = 0;
        Self::dfs(nums, 0, &mut ret);
        ret
    }

    fn dfs(mut nums: Vec<i32>, cur_idx: usize, ret: &mut i32) {
        if cur_idx >= nums.len() {
            *ret += 1;
        }
        for i in cur_idx..nums.len() {
            if i > cur_idx && nums[i] == nums[cur_idx] {
                continue;
            }
            nums.swap(i, cur_idx);
            if cur_idx == 0 || (cur_idx > 0 && Self::is_square(nums[cur_idx] + nums[cur_idx - 1])) {
                Self::dfs(nums.clone(), cur_idx + 1, ret);
            }
        }
    }

    fn is_square(v: i32) -> bool {
        let r = (v as f64).sqrt() as i32;
        r * r == v
    }
}

fn main() {}
