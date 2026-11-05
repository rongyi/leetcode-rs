struct Solution;

impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut current_position = 0;
        let mut boundary_hits = 0;

        for step in nums {
            current_position += step;

            // Check if we are back at the start after the move
            if current_position == 0 {
                boundary_hits += 1;
            }
        }

        boundary_hits
    }
}
fn main() {}
