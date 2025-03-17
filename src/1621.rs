#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as i64;
        let k = k as i64;

        // This problem can be solved using stars and bars method
        // It's equivalent to choosing 2*k positions from (n+k-1) positions
        Self::combination((n + k - 1) as usize, (2 * k) as usize, MOD) as i32
    }

    // Calculate C(n, k) mod m
    fn combination(n: usize, k: usize, m: i64) -> i64 {
        if k > n {
            return 0;
        }

        let k = k.min(n - k); // Optimization: C(n,k) = C(n,n-k)
        let mut result = 1i64;

        for i in 0..k {
            result = result * (n - i) as i64 % m;
            result = result * Self::mod_inverse((i + 1) as i64, m) % m;
        }

        result
    }

    // Calculate modular inverse using Fermat's Little Theorem
    fn mod_inverse(a: i64, m: i64) -> i64 {
        Self::mod_pow(a, m - 2, m)
    }

    // Fast modular exponentiation
    fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
        let mut result = 1;
        base %= m;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % m;
            }
            exp >>= 1;
            base = base * base % m;
        }

        result
    }
}
fn main() {}
