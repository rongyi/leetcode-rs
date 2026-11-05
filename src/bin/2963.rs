struct Solution;

impl Solution {
    pub fn number_of_good_partitions(nums: Vec<i32>) -> i32 {
        // We need to count the number of ways to partition the array into contiguous subarrays
        // such that each value appears in exactly one partition.
        // This is equivalent to finding the number of "cut points" between partitions.
        //
        // Approach:
        // 1. First, find the last occurrence index for each unique value in the array.
        //    This tells us the furthest position where each value appears.
        // 2. Scan through the array from left to right, tracking the current partition's end.
        //    For each element at index i, we update the partition end to be the maximum
        //    between the current end and the last occurrence of nums[i].
        // 3. When we reach index i that equals the current partition end, it means we've
        //    completed a valid partition where all occurrences of values in this segment
        //    are contained within it. We can cut here.
        // 4. Count how many such partitions we can form. Let's call this count `k`.
        //    The number of good partitions is actually 2^(k-1) because:
        //    - We have k segments, and we can choose to either cut or not cut between them.
        //    - But we must cut at the boundaries of these segments (they're determined by the algorithm).
        //    Actually, the algorithm finds exactly k segments where cutting is mandatory at their ends.
        //    Wait, let's clarify: The algorithm finds k segments where each is a minimal valid segment.
        //    Between these segments, we have k-1 gaps. For each gap, we can choose to either
        //    merge with the next segment or keep separate. That gives 2^(k-1) possibilities.
        //    However, the problem asks for the number of good partitions, which corresponds to
        //    the number of ways to choose cut points between these minimal segments.
        //    So if we have k minimal segments, we have k-1 gaps, and each gap can be cut or not.
        //    That's 2^(k-1) possibilities.
        // 5. Compute 2^(k-1) modulo 1_000_000_007.
        //
        // Example: nums = [1,2,3,4,5]
        // Last occurrences: all values appear only once at their positions.
        // Scanning: at i=0, end becomes 0 (last of 1), i==end → partition at index 0.
        // Similarly for each i, we get partitions at each index.
        // So k = 5, number of good partitions = 2^(4) = 16.
        //
        // Example: nums = [1,1,2,2]
        // Last occurrences: 1→1, 2→3.
        // i=0: end = max(0,1)=1, i!=end.
        // i=1: end = max(1,1)=1, i==end → partition at index 1 (first segment [1,1]).
        // i=2: end = max(1,3)=3, i!=end.
        // i=3: end = max(3,3)=3, i==end → partition at index 3 (second segment [2,2]).
        // So k=2, number of good partitions = 2^(1)=2.
        // The two partitions: [[1,1],[2,2]] and [[1,1,2,2]].
        //
        // Implementation steps below:
        use std::collections::HashMap;
        let mut last_occurrence = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            last_occurrence.insert(num, i);
        }
        let mut partitions = 0;
        let mut end = 0;
        let mut i = 0;
        while i < nums.len() {
            end = end.max(*last_occurrence.get(&nums[i]).unwrap());
            if i == end {
                partitions += 1;
            }
            i += 1;
        }
        let mod_val = 1_000_000_007;
        let mut result = 1;
        for _ in 1..partitions {
            result = (result * 2) % mod_val;
        }
        result
    }
}
fn main() {}
