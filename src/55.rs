struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut far_reach = 0;
        for (i, &step) in nums.iter().enumerate() {
            if i > far_reach {
                break;
            }
            far_reach = far_reach.max(i + step as usize);

            if far_reach >= nums.len() - 1 {
                return true;
            }
        }
        false
    }
}

fn main() {}
