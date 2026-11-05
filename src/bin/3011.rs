struct Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut prev_max = i32::MIN;
        let mut i = 0;
        let n = nums.len();

        while i < n {
            let current_bit_count = nums[i].count_ones();
            let mut curr_min = nums[i];
            let mut curr_max = nums[i];

            // Find the boundary of the current block
            let mut j = i + 1;
            while j < n && nums[j].count_ones() == current_bit_count {
                curr_min = curr_min.min(nums[j]);
                curr_max = curr_max.max(nums[j]);
                j += 1;
            }

            // If the smallest element in this block is smaller than
            // the largest element in the previous block, it's impossible.
            if curr_min < prev_max {
                return false;
            }

            // Update for the next block
            prev_max = curr_max;
            i = j;
        }

        true
    }
}
fn main() {}
