struct Solution;

impl Solution {
    /// LeetCode 852: Peak Index in a Mountain Array
    ///
    /// Array rises then falls. Find the peak in O(log n).
    ///
    /// Binary search on the slope:
    ///   arr[mid] < arr[mid+1] → still climbing → peak is to the right
    ///   arr[mid] > arr[mid+1] → started descending → peak is at or left
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, arr.len() - 1);

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if arr[mid] < arr[mid + 1] {
                lo = mid + 1; // climbing → peak is right of mid
            } else {
                hi = mid; // descending → peak is at or left of mid
            }
        }

        lo as i32
    }
}

fn main() {
    let tests = [
        (vec![0, 1, 0], 1),
        (vec![0, 2, 1, 0], 1),
        (vec![0, 10, 5, 2], 1),
        (vec![3, 4, 5, 1], 2),
    ];

    for (arr, expected) in &tests {
        let result = Solution::peak_index_in_mountain_array(arr.clone());
        println!(
            "{} arr={:?} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            arr,
            result,
            expected
        );
    }
}
