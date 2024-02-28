struct Solution;

impl Solution {
    // brute force can pass
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut sz = nums.len();
        if sz < 3 {
            return 0;
        }
        let mut ret = 0;

        for i in 0..sz {
            for j in i + 2..sz {
                if Self::is_arithmetic(&nums, i, j) {
                    ret += 1;
                }
            }
        }

        ret
    }

    fn is_arithmetic(nums: &[i32], i: usize, j: usize) -> bool {
        let delta = nums[i + 1] - nums[i];
        for k in i + 2..=j {
            if nums[k] - nums[k - 1] != delta {
                return false;
            }
        }
        true
    }
}

fn main() {}
