struct Solution;

impl Solution {
    /// LeetCode 330: Patching Array
    ///
    /// Given a sorted array `nums` and `n`, return the minimum number
    /// of patches needed so that every number in [1, n] can be formed
    /// as a sum of some elements.
    ///
    /// # Greedy approach
    ///
    /// Maintain `miss` = the smallest number we CANNOT form yet.
    /// Initially we can form [0, 0], so miss = 1.
    ///
    /// For each num in nums:
    ///   - If num ≤ miss: we now cover [0, miss+num)
    ///                    → miss += num, move to next num.
    ///   - If num > miss: we can't reach miss! Patch with miss itself.
    ///                    After patching, cover [0, 2·miss)
    ///                    → miss *= 2, patches++.
    ///
    /// After exhausting nums, keep patching (doubling miss) until miss > n.
    ///
    /// Why patch with `miss`? Any number > miss leaves a gap. Any number
    /// < miss extends less. So `miss` is the optimal greedy choice.
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut miss = 1i64;     // smallest unreachable sum
        let mut patches = 0;
        let mut i = 0;
        let sz = nums.len();

        while miss <= n as i64 {
            if i < sz && nums[i] as i64 <= miss {
                // Can use this num to extend coverage.
                miss += nums[i] as i64;
                i += 1;
            } else {
                // Must patch miss to cover the gap.
                miss += miss;
                patches += 1;
            }
        }

        patches
    }
}

fn main() {
    let tests = [
        (vec![1, 3], 6, 1),
        (vec![1, 5, 10], 20, 2),
        (vec![1, 2, 2], 5, 0),
        (vec![], 8, 4),   // need 1,2,4,8 — but 4 patches? Let's check:
                          // miss=1→patch 1→miss=2→patch 2→miss=4→patch 4→miss=8→patch 8→miss=16 > 8 = 4 ✓
        (vec![1, 2, 31, 33], 2147483647, 28),
    ];

    for (nums, n, expected) in &tests {
        let result = Solution::min_patches(nums.clone(), *n);
        println!(
            "{} nums={:?} n={} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            nums,
            n,
            result,
            expected
        );
    }
}
