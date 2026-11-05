struct Solution;

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut r = 1e9 as i32;
        while l < r {
            let mid = l + (r - l) / 2;
            if Self::check(&nums, k, mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        l
    }

    // check all num <= vall and not by side, and total number <= k
    fn check(nums: &Vec<i32>, mut k: i32, val: i32) -> bool {
        let mut i = 0;
        let sz = nums.len();
        while i < sz {
            if nums[i] <= val {
                k -= 1;
                i += 2;
            } else {
                i += 1;
            }
            if k == 0 {
                return true;
            }
        }
        k == 0
    }
}

fn main() {}
