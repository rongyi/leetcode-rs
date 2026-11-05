struct Solution;

impl Solution {
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        let sz = nums.len();
        nums.sort_unstable();
        for i in 0..sz - 1 {
            if nums[i] != (i + 1) as i32 {
                return false;
            }
        }
        nums[sz - 1] == sz as i32 - 1
    }
}

fn main() {}
