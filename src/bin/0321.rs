struct Solution;

impl Solution {
    /// LeetCode 321: Create Maximum Number
    ///
    /// Given two digit arrays and k, return the maximum number of length k
    /// formed by picking digits from both arrays while preserving order within each.
    ///
    /// Approach: for every possible split (i from nums1, k-i from nums2),
    ///   1. Pick the maximum subsequence of length i from nums1.
    ///   2. Pick the maximum subsequence of length k-i from nums2.
    ///   3. Merge them greedily, picking the larger leading digit.
    ///   4. Keep the lexicographically largest result.
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let (n1, n2) = (nums1.len() as i32, nums2.len() as i32);
        let mut best: Vec<i32> = Vec::new();

        // k1 = how many to take from nums1.
        // Lower bound: we must take at least k - n2 from nums1 (if k > n2).
        // Upper bound: we can take at most k from nums1, but no more than n1.
        for k1 in (k - n2).max(0)..=k.min(n1) {
            let a = Self::max_subsequence(&nums1, k1);
            let b = Self::max_subsequence(&nums2, k - k1);
            let merged = Self::merge(&a, &b);
            if Self::is_greater(&merged, &best) {
                best = merged;
            }
        }

        best
    }

    /// Picks the lexicographically largest subsequence of length `k` from `nums`.
    /// Uses a monotonic stack: drop n-k smaller elements from the "inside".
    fn max_subsequence(nums: &[i32], k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut drop = nums.len() - k;
        let mut stack: Vec<i32> = Vec::with_capacity(k);

        for &num in nums {
            while !stack.is_empty() && drop > 0 && *stack.last().unwrap() < num {
                stack.pop();
                drop -= 1;
            }
            stack.push(num);
        }

        stack.truncate(k);
        stack
    }

    /// Merges two sequences into the lexicographically largest result.
    /// At each step, picks from whichever sequence has the larger suffix.
    fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
        let mut result = Vec::with_capacity(a.len() + b.len());
        let (mut i, mut j) = (0, 0);

        while i < a.len() || j < b.len() {
            if Self::greater_suffix(a, i, b, j) {
                result.push(a[i]);
                i += 1;
            } else {
                result.push(b[j]);
                j += 1;
            }
        }

        result
    }

    /// Returns true if suffix a[i..] is lexicographically greater than b[j..].
    fn greater_suffix(a: &[i32], i: usize, b: &[i32], j: usize) -> bool {
        let (mut p, mut q) = (i, j);
        while p < a.len() && q < b.len() {
            if a[p] > b[q] {
                return true;
            } else if a[p] < b[q] {
                return false;
            }
            p += 1;
            q += 1;
        }
        // All compared elements were equal — the longer suffix wins
        // because having more elements gives flexibility.
        p < a.len()
    }

    /// Lexicographic comparison of two slices.
    fn is_greater(a: &[i32], b: &[i32]) -> bool {
        for i in 0..a.len().min(b.len()) {
            if a[i] > b[i] {
                return true;
            } else if a[i] < b[i] {
                return false;
            }
        }
        a.len() > b.len()
    }
}

fn main() {}
