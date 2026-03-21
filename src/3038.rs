struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        // The very first operation sets the target score
        let target_score = nums[0] + nums[1];
        let mut operations = 1;

        // Start checking from the next pair (index 2 and 3)
        let mut i = 2;
        while i + 1 < nums.len() {
            if nums[i] + nums[i + 1] == target_score {
                operations += 1;
                i += 2; // Move to the next pair
            } else {
                // The score changed, so we must stop immediately
                break;
            }
        }

        operations
    }
}

fn main() {}
