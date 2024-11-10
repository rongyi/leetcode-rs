struct Solution;

/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct MountainArray;
 *  impl MountainArray {
 *     fn get(index:i32)->i32;
 *     fn length()->i32;
 * };
 */
struct MountainArray;
impl MountainArray {
    fn get(&self, index: i32) -> i32 {
        0
    }
    fn length(&self) -> i32 {
        0
    }
}

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        let sz = mountain_arr.length();
        let mut left = 0;
        let mut right = sz - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            if mountain_arr.get(mid) < mountain_arr.get(mid + 1) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        let peek = left;
        // search in left side
        left = 0;
        right = peek;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = mountain_arr.get(mid);
            if mid_val == target {
                return mid;
            } else if mid_val < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        // search in right side
        left = peek;
        right = sz - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = mountain_arr.get(mid);
            if mid_val == target {
                return mid;
            } else if mid_val > target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}

fn main() {}
