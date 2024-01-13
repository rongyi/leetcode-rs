struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let sz = nums.len();
        let k = k as usize % sz;
        nums.reverse();

        nums[..k].reverse();
        nums[k..].reverse();
    }
}

fn main() {}
