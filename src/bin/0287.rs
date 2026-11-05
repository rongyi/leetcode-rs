struct Solution;

impl Solution {
    /// LeetCode 287: Find the Duplicate Number
    ///
    /// Given an array of `n + 1` integers in the range `[1, n]`,
    /// find the only duplicate number.
    ///
    /// Uses Floyd's Tortoise and Hare cycle detection:
    /// - Treat `nums[i]` as a pointer from index `i` to index `nums[i]`.
    /// - The duplicate creates a cycle in this implicit linked list.
    /// - Time: O(n), Space: O(1), no array modification.
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // Phase 1: Find the intersection point inside the cycle.
        let mut slow = nums[0];
        let mut fast = nums[0];

        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break;
            }
        }

        // Phase 2: Find the entrance to the cycle (the duplicate).
        let mut slow2 = nums[0];
        while slow2 != slow {
            slow2 = nums[slow2 as usize];
            slow = nums[slow as usize];
        }

        slow
    }
}

fn main() {}
