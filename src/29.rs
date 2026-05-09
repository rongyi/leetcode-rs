struct Solution;

use std::{i32, ops::Neg};
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        let mut ret: i64 = 0;
        let is_neg = dividend.is_negative() ^ divisor.is_negative();
        let mut a: i64 = (dividend as i64).abs();
        let b: i64 = (divisor as i64).abs();

        while a >= b {
            let mut quot = 1;
            let mut cur = b;
            while (cur << 1) > cur && a >= (cur << 1) {
                cur <<= 1;
                quot <<= 1;
            }

            ret += quot;
            a -= cur;
        }

        if is_neg {
            ret.neg() as i32
        } else {
            ret as i32
        }
    }
}

fn main() {
    println!("{}", i32::MIN);
    let val = Solution::divide(-2147483648, 2);
    println!("{val}");
}
