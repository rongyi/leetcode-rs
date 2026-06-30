pub struct Solution;

impl Solution {
    /// LeetCode 365: Water and Jug Problem
    ///
    /// Two jugs of capacities a and b. Can we measure exactly `target`?
    ///
    /// # Approach: Bézout's identity
    ///
    /// The operations (fill, empty, pour) let us add ±a or ±b at any time.
    /// So the reachable amounts are exactly the integer combinations:
    ///
    ///   a·x + b·y  for integers x, y
    ///
    /// By Bézout's identity, the set of all a·x + b·y is exactly the
    /// multiples of gcd(a, b). And the amount can't exceed a + b.
    ///
    /// Therefore: target is reachable ⇔
    ///   target ≤ a + b   AND   target % gcd(a, b) == 0
    pub fn can_measure_water(a: i32, b: i32, target: i32) -> bool {
        if target > a + b {
            return false;
        }
        if target == 0 {
            return true;
        }
        target % Self::gcd(a, b) == 0
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        Self::gcd(b, a % b)
    }
}

fn main() {
    let tests = [
        (3, 5, 4, true), // fill 5, pour to 3 → 2 left in 5; empty 3, pour 2 in;
        // fill 5, pour to 3 (needs 1) → 4 left in 5 ✓
        (2, 6, 5, false), // gcd(2,6)=2, 5%2=1 ≠ 0
        (1, 2, 3, true),  // 3 ≤ 1+2=3, 3%1=0 ✓
        (4, 6, 8, true),  // gcd=2, 8%2=0, 8 ≤ 10 ✓
    ];

    for (a, b, target, expected) in &tests {
        let result = Solution::can_measure_water(*a, *b, *target);
        println!(
            "{} a={} b={} target={} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            a,
            b,
            target,
            result,
            expected
        );
    }
}
