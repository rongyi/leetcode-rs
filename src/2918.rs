struct Solution;

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut sum1: i64 = 0;
        let mut sum2: i64 = 0;
        let mut zero1: i64 = 0;
        let mut zero2: i64 = 0;

        // Process first array
        for &num in &nums1 {
            if num == 0 {
                zero1 += 1;
            } else {
                sum1 += num as i64;
            }
        }

        // Process second array
        for &num in &nums2 {
            if num == 0 {
                zero2 += 1;
            } else {
                sum2 += num as i64;
            }
        }

        let min_sum1 = sum1 + zero1;
        let min_sum2 = sum2 + zero2;

        // Case 1: Both sums are equal - the answer is the sum
        if min_sum1 == min_sum2 {
            return min_sum1;
        }

        // Case 2: Array 1 has a larger min_sum
        if min_sum1 > min_sum2 {
            // We can only increase Array 2 if it has at least one zero
            if zero2 > 0 {
                return min_sum1;
            } else {
                return -1;
            }
        }

        // Case 3: Array 2 has a larger min_sum
        if min_sum2 > min_sum1 {
            // We can only increase Array 1 if it has at least one zero
            if zero1 > 0 {
                return min_sum2;
            } else {
                return -1;
            }
        }

        -1
    }
}

fn main() {}
