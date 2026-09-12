struct Solution;

impl Solution {
    /// LeetCode 1009: Complement of Base 10 Integer
    ///
    /// Flip all bits of n's binary representation (ignoring leading zeros).
    ///
    ///   5 = 101  → 010 = 2
    ///   7 = 111  → 000 = 0
    ///
    /// Flip = XOR with a mask of all 1s spanning n's significant bits.
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1; // "0" → "1"
        }
        // Number of significant bits (ignoring leading zeros).
        let bits = 32 - n.leading_zeros();
        // Mask with `bits` ones, e.g. bits=3 → 111.
        let mask = (1i32 << bits) - 1;
        n ^ mask
    }
}

fn main() {
    let tests = [(5, 2), (7, 0), (10, 5), (0, 1), (1, 0), (2, 1)];

    for (n, expected) in &tests {
        let result = Solution::bitwise_complement(*n);
        println!(
            "{} n={:2} ({:b}) → {:2} ({:b}) (expected {})",
            if result == *expected { "✓" } else { "✗" },
            n,
            n,
            result,
            result,
            expected
        );
    }
}
