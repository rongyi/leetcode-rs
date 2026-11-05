#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // Calculate (x^n) % MOD using fast power algorithm
        fn pow_mod(mut x: i64, mut n: i64, modulus: i64) -> i64 {
            let mut result = 1;
            x = x % modulus;

            while n > 0 {
                if n % 2 == 1 {
                    result = (result * x) % modulus;
                }
                x = (x * x) % modulus;
                n /= 2;
            }

            result
        }

        // For even positions (0, 2, 4...), we can use digits 0, 2, 4, 6, 8 (5 options)
        // For odd positions (1, 3, 5...), we can use digits 1, 3, 5, 7, 9 (5 options)

        // Calculate how many even and odd positions we have
        let even_positions = (n + 1) / 2; // Ceiling division for even positions
        let odd_positions = n / 2; // Floor division for odd positions

        // Calculate 5^even_positions and 4^odd_positions
        let even_contribution = pow_mod(5, even_positions, MOD);
        let odd_contribution = pow_mod(4, odd_positions, MOD);

        // Final result is (5^even_positions * 4^odd_positions) % MOD
        ((even_contribution * odd_contribution) % MOD) as i32
    }
}
fn main() {}
