struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = (m + n - 1) as usize;
        while i >= 0 && j >= 0 {
            let v1 = nums1[i as usize];
            let v2 = nums2[j as usize];
            if v1 <= v2 {
                nums1[k] = v2;
                j -= 1;
            } else {
                nums1[k] = v1;
                i -= 1;
            }

            k -= 1;
        }

        while j >= 0 {
            nums1[k] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}

fn main() {}
