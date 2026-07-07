struct Solution;

impl Solution {
    /// LeetCode 396: Rotate Function
    ///
    /// F(k) = Σ i * nums[(k+i) % n] for i=0..n-1.
    /// Return the max over all k.
    ///
    /// # Derivation
    ///
    /// F(0) = 0·n₀ + 1·n₁ + 2·n₂ + ... + (n-1)·nₙ₋₁
    ///
    /// After one left rotation: every element's weight increases by 1,
    /// except n₀ which wraps from weight n-1 to 0.
    ///
    ///   F(1) = F(0) + n·n₀ - sum
    ///   F(2) = F(1) + n·n₁ - sum
    ///   ...
    ///
    /// General:  F(k) = F(k-1) + n·nums[k-1] - sum
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();

        // F(0)
        let mut f: i64 = nums
            .iter()
            .enumerate()
            .map(|(i, &x)| i as i64 * x as i64)
            .sum();

        let mut ans = f;
        for k in 1..n {
            f += n as i64 * nums[k - 1] as i64 - sum;
            ans = ans.max(f);
        }

        ans as i32
    }
}

fn main() {
    let tests = [
        (vec![4, 3, 2, 6], 26),
        (vec![100], 0),
        (vec![-1, -2, -3], -5),
    ];

    for (nums, expected) in &tests {
        let result = Solution::max_rotate_function(nums.clone());
        println!(
            "{} nums={:?} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            nums,
            result,
            expected
        );
    }
}
