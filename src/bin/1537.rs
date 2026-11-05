#![allow(dead_code)]

struct Solution;

impl Solution {
    // This solution is using a two-pointer approach to find the maximum sum path
    // through two arrays, where we can switch between arrays at "intersection points".
    //
    // The algorithm works by:
    // 1. Simultaneously scanning both arrays (nums1 and nums2) using pointers i and j
    // 2. For each step, we have three possibilities:
    //    - Move forward in nums1 if its current element is smaller
    //    - Move forward in nums2 if its current element is smaller
    //    - If elements are equal (intersection point), we choose the maximum sum path so far
    //      and continue from that path
    //
    // At each intersection point, we update both sums to the maximum sum path,
    // essentially "merging" the paths at that point.
    //
    // After traversing both arrays, we return the maximum of the two sums,
    // modulo 10^9 + 7 as specified in the problem.
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let n = nums1.len();
        let m = nums2.len();

        let mod_val = 1_000_000_007;

        let mut sum1: i64 = 0;
        let mut sum2: i64 = 0;

        while i < n || j < m {
            if i < n && (j == m || nums1[i] < nums2[j]) {
                sum1 += nums1[i] as i64;
                i += 1;
            } else if j < m && (i == n || nums1[i] > nums2[j]) {
                sum2 += nums2[j] as i64;
                j += 1;
            } else {
                // At intersection, choose the maximum sum path
                sum1 = sum1.max(sum2) + nums1[i] as i64;
                sum2 = sum1;
                i += 1;
                j += 1;
            }
        }
        (sum1.max(sum2) % mod_val) as i32
    }
}
fn main() {}
