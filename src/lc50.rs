struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let mut ret = Self::my_pow(x, n / 2);
        ret *= ret;
        if n % 2 != 0 {
            if n > 0 {
                ret *= x;
            } else {
                ret /= x;
            }
        }

        ret
    }
}
