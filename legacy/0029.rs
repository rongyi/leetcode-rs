struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        let sign = if (dividend < 0) ^ (divisor < 0) {
            -1
        } else {
            1
        };
        let mut dd = (dividend as i64).abs();
        let ds = (divisor as i64).abs();
        let mut ret = 0;

        while dd >= ds {
            let mut tmp = ds;
            let mut multiple = 1;
            while dd >= (tmp << 1) {
                tmp <<= 1;
                multiple <<= 1;
            }
            dd -= tmp;
            ret += multiple;
        }

        sign * ret as i32
    }
}
