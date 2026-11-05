struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        for i in (0..nums.len() - 2).rev() {
            if nums[i] + nums[i + 1] > nums[i + 2] {
                return nums[i] + nums[i + 1] + nums[i + 2];
            }
        }
        0
    }
}
fn main() {}
