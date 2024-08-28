struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = arr.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            // If arr[mid] < arr[mid + 1], we're on the ascending slope, so the peak must be to the right.
            // If arr[mid] > arr[mid + 1], we're either at the peak or on the descending slope, so the peak must be at mid or to the left.
            if arr[mid] < arr[mid + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as i32
    }
}

fn main() {}
