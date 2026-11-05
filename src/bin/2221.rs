struct Solution;

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        while nums.len() > 1 {
            let mut next_layer = vec![];
            for i in 0..nums.len() - 1 {
                next_layer.push((nums[i] + nums[i + 1]) % 10);
            }
            nums = next_layer;
        }
        nums[0]
    }
}

fn main() {}
