struct Solution;

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        // Cast to i64 immediately to prevent overflow during multiplication
        let n = n as i64;
        let m = m as i64;

        let odd_n = (n + 1) / 2;
        let even_n = n / 2;

        let odd_m = (m + 1) / 2;
        let even_m = m / 2;

        // Alice wins if (x is odd and y is even) OR (x is even and y is odd)
        (odd_n * even_m) + (even_n * odd_m)
    }
}

fn main() {}
