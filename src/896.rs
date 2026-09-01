struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut is_mono_increse = true;
        let mut is_mono_decrese = true;
        let sz = nums.len();
        for i in 1..sz {
            if nums[i] < nums[i - 1] {
                is_mono_increse = false;
                break;
            }
        }
        for i in 1..sz {
            if nums[i] > nums[i - 1] {
                is_mono_decrese = false;
                break;
            }
        }

        is_mono_increse || is_mono_decrese
    }
}

fn main() {}
