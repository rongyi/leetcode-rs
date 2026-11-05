struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        // assume num1 is smaller
        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
        let mut low = 0;
        let mut high = m;
        while low <= high {
            let cutx = (low + high) / 2;
            let cuty = (m + n + 1) / 2 - cutx; // make total left part one more when in odd case

            let max_left_x = if cutx == 0 { i32::MIN } else { nums1[cutx - 1] };

            let min_right_x = if cutx == m { i32::MAX } else { nums1[cutx] };

            let max_left_y = if cuty == 0 { i32::MIN } else { nums2[cuty - 1] };
            let min_right_y = if cuty == n { i32::MAX } else { nums2[cuty] };

            if max_left_x <= min_right_y && max_left_y <= min_right_x {
                if (m + n) % 2 == 0 {
                    return (max_left_x.max(max_left_y) + min_right_x.min(min_right_y)) as f64
                        / 2.0;
                } else {
                    return max_left_x.max(max_left_y) as f64;
                }
            } else if max_left_x > min_right_y {
                high = cutx - 1;
            } else {
                low = cutx + 1;
            }
        }

        0.0
    }
}

fn main() {}
