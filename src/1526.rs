#![allow(dead_code)]
struct Solution;

impl Solution {
    // The key insight to this problem is understanding how the operations accumulate:

    // 1. To build the target array, we need to think "bottom-up", considering how values are built from zero.

    // 2. For the first element, we need exactly `target[0]` operations to raise it from 0 to its target value.

    // 3. For subsequent elements:
    //    - If the current element `target[i]` is less than or equal to the previous element `target[i-1]`, we don't need additional operations. This is because when we increment the previous element, we can also increment the current element as part of the same subarray operation.
    //    - If the current element is greater than the previous, we need `target[i] - target[i-1]` additional operations to raise it to its target value.

    // This greedy approach works because:
    // - We always want to "reuse" as many increments as possible from previous elements
    // - The constraint of operating on subarrays means we can only increment contiguous elements together

    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut prev = 0;

        for &num in &target {
            if num > prev {
                result += num - prev;
            }
            prev = num;
        }

        result
    }
}

fn main() {}
