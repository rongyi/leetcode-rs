struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n <= 2 {
            return 1;
        }

        let mut x0 = 0;
        let mut x1 = 1;
        let mut x2 = 1;
        let mut acc = 0;
        for _ in 3..=n {
            acc = x0 + x1 + x2;

            x0 = x1;
            x1 = x2;
            x2 = acc;
        }

        acc
    }
}

fn main() {}
