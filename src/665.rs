struct Solution;

impl Solution {
    /// LeetCode 665: Non-decreasing Array
    ///
    /// Can we make the array non-decreasing with at most ONE modification?
    ///
    /// # Why two passes?
    ///
    /// A single modification can fix at most one "drop" (left scan) AND
    /// at most one "rise" (right scan). If both scans find > 1 problem,
    /// one edit can't fix both ends → false.
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        // Left pass: count elements smaller than the running max (drops).
        let mut drops = 0;
        let mut max_so_far = i32::MIN;
        for &num in &nums {
            if num >= max_so_far {
                max_so_far = num;
            } else {
                drops += 1;
            }
        }

        // Right pass: count elements larger than the running min (rises).
        let mut rises = 0;
        let mut min_so_far = i32::MAX;
        for &num in nums.iter().rev() {
            if num <= min_so_far {
                min_so_far = num;
            } else {
                rises += 1;
            }
        }

        !(drops > 1 && rises > 1)
    }
}

fn main() {
    let tests = [
        (vec![4, 2, 3], true),
        (vec![4, 2, 1], false),
        (vec![3, 4, 2, 3], false),
        (vec![1, 2, 5, 3, 4], true),
        (vec![3, 3, 2, 2], false),
        (vec![1], true),
    ];

    for (nums, expected) in &tests {
        let result = Solution::check_possibility(nums.clone());
        println!(
            "{} nums={:?} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            nums,
            result,
            expected
        );
    }
}
