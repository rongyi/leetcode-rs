struct Solution;

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let sz = nums.len();
        let p1 = nums[sz - 1] * nums[sz - 2] * nums[sz - 3];
        let p2 = nums[0] * nums[1] * nums[sz - 1];
        p1.max(p2)
    }
}

fn main() {}
