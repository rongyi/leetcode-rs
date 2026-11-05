struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        // We use i64 because the combinations can exceed the i32 range
        let n = n as i64;
        let limit = limit as i64;

        // Helper function for Combinations(m, 2)
        // Formula: m! / (2! * (m-2)!) = (m * (m-1)) / 2
        fn c2(m: i64) -> i64 {
            if m < 2 {
                return 0;
            }
            (m * (m - 1)) / 2
        }

        let mut ans = c2(n + 2); // Total ways

        // 1 child exceeds: n - (limit + 1)
        ans -= 3 * c2(n - limit - 1 + 2);

        // 2 children exceed: n - 2*(limit + 1)
        ans += 3 * c2(n - 2 * limit - 2 + 2);

        // 3 children exceed: n - 3*(limit + 1)
        ans -= c2(n - 3 * limit - 3 + 2);

        ans
    }
}

fn main() {}
