pub struct Solution;

impl Solution {
    /// LeetCode 367: Valid Perfect Square
    ///
    /// Returns true if `num` is a perfect square, without using sqrt.
    ///
    /// # Approach: Binary search
    ///
    /// Search for x in [1, num] where x² = num.
    /// Use i64 to avoid overflow on mid*mid.
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let (mut lo, mut hi) = (1i64, num);

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let sq = mid * mid;
            match sq.cmp(&num) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid - 1,
            }
        }

        false
    }
}

fn main() {
    let tests = [(16, true), (14, false), (1, true), (808201, true)];

    for (num, expected) in &tests {
        let result = Solution::is_perfect_square(*num);
        println!(
            "{} num={} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            num,
            result,
            expected
        );
    }
}
