#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        // Empty array case
        if n == 0 {
            return 0;
        }

        // Sort the array
        let mut sorted_arr = arr;
        sorted_arr.sort_unstable();

        // Set the first element to 1 if it's greater
        if sorted_arr[0] > 1 {
            sorted_arr[0] = 1;
        }

        // Process each element
        for i in 1..n {
            // If current element is greater than previous+1, set it to previous+1
            if sorted_arr[i] > sorted_arr[i - 1] + 1 {
                sorted_arr[i] = sorted_arr[i - 1] + 1;
            }
        }

        // Return the maximum element, which is the last element of the sorted array
        sorted_arr[n - 1]
    }
}

fn main() {}
