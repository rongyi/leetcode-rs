#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        // We need to find a value k (0 <= k < 2^maximumBit) for each position i
        // such that the XOR of k with the XOR of all elements from nums[0] to nums[i] is maximized

        let n = nums.len();
        let mut answer = vec![0; n];
        let max_value = (1 << maximum_bit) - 1; // 2^maximum_bit - 1
        let mut running_xor = 0;

        // Process the array in reverse order
        for i in 0..n {
            running_xor ^= nums[i];

            // To maximize the XOR, we need a number that differs in all bits from running_xor
            // Since we're limited to maximum_bit bits, we use k = max_value ^ running_xor
            answer[n - 1 - i] = max_value ^ running_xor;
        }

        answer
    }
}
fn main() {}
