struct Solution;

impl Solution {
    /// LeetCode 319: Bulb Switcher
    ///
    /// n bulbs, initially off. Round i toggles every i-th bulb.
    /// How many bulbs are on after n rounds?
    ///
    /// # Why it's just √n
    ///
    /// Bulb k is toggled in every round i where i divides k.
    /// So bulb k is toggled τ(k) times (the number of divisors of k).
    ///
    /// Divisors come in pairs:  a × b = k.
    /// Each pair gives 2 divisors... except when a = b (a perfect square).
    ///
    ///   k = 12:  1×12, 2×6, 3×4  → 6 divisors  → OFF (even)
    ///   k = 16:  1×16, 2×8, 4×4   → 5 divisors  → ON  (odd)
    ///
    /// Only perfect squares have an odd number of divisors.
    /// So the answer is the count of perfect squares ≤ n = ⌊√n⌋.
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

fn main() {
    let tests = [(0, 0), (1, 1), (3, 1), (4, 2), (10, 3), (100, 10)];

    for (n, expected) in &tests {
        let result = Solution::bulb_switch(*n);
        println!(
            "{} n={:3} → sqrt({}) = {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            n,
            n,
            result,
            expected
        );
    }
}
