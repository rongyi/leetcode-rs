struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut min1 = i32::MAX; // Smallest number
        let mut min2 = i32::MAX; // Second smallest number

        let mut max1 = i32::MIN; // Largest number
        let mut max2 = i32::MIN; // Second largest number
        let mut max3 = i32::MIN; // Third largest number

        for &n in &nums {
            // Update the top 3 maximums
            if n > max1 {
                max3 = max2;
                max2 = max1;
                max1 = n;
            } else if n > max2 {
                max3 = max2;
                max2 = n;
            } else if n > max3 {
                max3 = n;
            }

            // Update the top 2 minimums
            if n < min1 {
                min2 = min1;
                min1 = n;
            } else if n < min2 {
                min2 = n;
            }
        }

        // Compare option 1 (three largest) vs. option 2 (two smallest negatives * largest)
        (max1 * max2 * max3).max(min1 * min2 * max1)
    }
}

fn main() {}
