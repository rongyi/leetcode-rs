#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let mut lo = 1i64;
        let mut hi = 2 * (1e9 as i64);

        let a = a as i64;
        let b = b as i64;
        let c = c as i64;
        let n = n as i64;
        let ab = a * b / Self::gcd(a, b);
        let bc = b * c / Self::gcd(b, c);
        let ac = a * c / Self::gcd(a, c);
        let abc = a * bc / Self::gcd(a, bc);

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let cnt = mid / a + mid / b + mid / c - mid / ab - mid / bc - mid / ac + mid / abc;
            if cnt < n {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        lo as i32
    }
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let val = a % b;
            a = b;
            b = val;
        }
        a
    }
}

fn main() {}
