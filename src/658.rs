struct Solution;

impl Solution {
    /// LeetCode 658: Find K Closest Elements
    ///
    /// Two-pointer shrinking window. Start with the full array,
    /// repeatedly remove the boundary element that is FARTHER from x
    /// (tie → remove the right one, since left is smaller).
    /// When k elements remain, they are the k closest.
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (mut lo, mut hi) = (0i32, arr.len() as i32 - 1);

        while hi - lo >= k {
            if (arr[lo as usize] - x).abs() > (arr[hi as usize] - x).abs() {
                lo += 1; // left element is farther → drop it
            } else {
                hi -= 1; // right element is farther (or tie) → drop it
            }
        }

        arr[lo as usize..=hi as usize].to_vec()
    }
}

fn main() {
    let tests = [
        (vec![1, 2, 3, 4, 5], 4, 3, vec![1, 2, 3, 4]),
        (vec![1, 2, 3, 4, 5], 4, -1, vec![1, 2, 3, 4]),
        (vec![1, 2, 3, 4, 5], 2, 3, vec![2, 3]),
        (vec![1, 2, 3, 4, 5], 2, 100, vec![4, 5]),
    ];

    for (arr, k, x, expected) in &tests {
        let result = Solution::find_closest_elements(arr.clone(), *k, *x);
        println!(
            "{} arr={:?} k={} x={} → {:?} (expected {:?})",
            if result == *expected { "✓" } else { "✗" },
            arr,
            k,
            x,
            result,
            expected
        );
    }
}
