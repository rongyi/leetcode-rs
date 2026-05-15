struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        Self::mypow(x, n as i64)
    }
    fn mypow(x: f64, n: i64) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n < 0 {
            return 1.0 / Self::mypow(x, -n);
        }
        if n % 2 == 1 {
            return Self::mypow(x, n - 1) * x;
        }
        let v = Self::mypow(x, n / 2);
        v * v
    }
}

fn main() {}
