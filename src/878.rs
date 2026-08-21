struct Solution;

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn gcd(mut x: i64, mut y: i64) -> i64 {
            while y != 0 {
                let temp = y;
                y = x % y;
                x = temp;
            }
            x
        }

        fn lcm(x: i64, y: i64) -> i64 {
            x / gcd(x, y) * y
        }

        let a = a as i64;
        let b = b as i64;
        let n = n as i64;
        let l = lcm(a, b);

        // Number of magical numbers in one LCM cycle
        let cycle_count = l / a + l / b - 1;
        let full_cycles = n / cycle_count;
        let remainder = n % cycle_count;

        // Base value from full cycles
        let base = (full_cycles % MOD) * (l % MOD) % MOD;

        if remainder == 0 {
            return base as i32;
        }

        // Find the remainder-th magical number within one cycle
        let mut left = 0;
        let mut right = l;

        while left < right {
            let mid = left + (right - left) / 2;
            let count = mid / a + mid / b;
            if count < remainder {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        ((base + left) % MOD) as i32
    }
}

fn main() {}
