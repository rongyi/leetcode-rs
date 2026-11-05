struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut prefixes = HashSet::new();

        // Step 1: Build the prefix set from arr1
        for mut num in arr1 {
            while num > 0 {
                prefixes.insert(num);
                num /= 10; // Integer division strips the last digit (acts as prefixing)
            }
        }

        let mut max_len = 0;

        // Step 2: Check arr2 against the set
        for mut num in arr2 {
            while num > 0 {
                if prefixes.contains(&num) {
                    // Calculate length: log10 + 1 is the digit count
                    let current_len = (num as f64).log10().floor() as i32 + 1;
                    max_len = max_len.max(current_len);

                    // Optimization: if we found a prefix for this number,
                    // smaller prefixes of the same number won't beat max_len.
                    break;
                }
                num /= 10;
            }
        }

        max_len
    }
}
fn main() {}
