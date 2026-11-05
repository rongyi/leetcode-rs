struct Solution;

impl Solution {
    pub fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = 1;

        // ending with nums1[i]
        let mut dp1 = 1;
        // ending with nums2[i]
        let mut dp2 = 1;
        let sz = nums1.len();
        for i in 1..sz {
            let mut cur_dp1 = 1;
            let mut cur_dp2 = 1;
            if nums1[i] >= nums1[i - 1] {
                cur_dp1 = cur_dp1.max(1 + dp1);
            }
            if nums1[i] >= nums2[i - 1] {
                cur_dp1 = cur_dp1.max(1 + dp2);
            }

            if nums2[i] >= nums2[i - 1] {
                cur_dp2 = cur_dp2.max(1 + dp2);
            }
            if nums2[i] >= nums1[i - 1] {
                cur_dp2 = cur_dp2.max(1 + dp1);
            }

            dp1 = cur_dp1;
            dp2 = cur_dp2;

            dp = dp.max(dp1.max(dp2));
        }

        dp
    }
}

fn main() {}
