struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let sz = nums.len();
        let mut increase: Option<bool> = None;
        let mut decrease: Option<bool> = None;
        for i in 1..sz {
            if nums[i] > nums[i - 1] {
                if decrease.is_some() {
                    return false;
                }
                increase = Some(true);
            }
            if nums[i] < nums[i - 1] {
                if increase.is_some() {
                    return false;
                }
                decrease = Some(true);
            }
        }

        true
    }
}

fn main() {}
