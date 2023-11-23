struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ret = Vec::new();
        for i in 0..nums.len() - 2 {
            // dedup
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == 0 {
                    ret.push(vec![nums[i], nums[l], nums[r]]);
                    // dedup
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    // dedup
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    // find one pair
                    l += 1;
                    r -= 1;
                } else if sum > 0 {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }

        ret
    }
}
