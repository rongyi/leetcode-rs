struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ret = vec![];
        let sz = nums.len();

        for i in 0..sz - 2 {
            // no more valid
            if nums[i] > 0 {
                break;
            }
            // ignore duplicate
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = sz - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == 0 {
                    ret.push(vec![nums[i], nums[l], nums[r]]);
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                } else if sum < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }

        ret
    }
}

fn main() {}
