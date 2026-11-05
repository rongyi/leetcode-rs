struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let sz = nums.len();
        nums.sort_unstable();
        let mut ret = vec![];
        let target = target as i64;

        for i in 0..=sz - 4 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..=sz - 3 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut l = j + 1;
                let mut r = sz - 1;
                while l < r {
                    let sum: i64 =
                        nums[i] as i64 + nums[j] as i64 + nums[l] as i64 + nums[r] as i64;
                    if sum == target {
                        ret.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
                    } else if sum < target {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
            }
        }

        ret
    }
}

fn main() {}
