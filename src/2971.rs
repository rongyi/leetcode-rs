struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        // Step 1: Sort the sides in ascending order
        nums.sort_unstable();

        // Step 2: Calculate the total sum of all elements
        // We use i64 to prevent overflow during summation
        let mut current_sum: i64 = nums.iter().map(|&x| x as i64).sum();

        // Step 3: Iterate backwards from the largest element
        // We need at least 3 sides, so we stop when the index is 2
        for i in (2..nums.len()).rev() {
            let longest_side = nums[i] as i64;
            let sum_of_others = current_sum - longest_side;

            // The condition for a polygon: longest side < sum of others
            if sum_of_others > longest_side {
                return current_sum;
            }

            // If the current largest side doesn't work, discard it
            // and update the sum for the next iteration
            current_sum -= longest_side;
        }

        // If no polygon can be formed
        -1
    }
}
fn main() {}
