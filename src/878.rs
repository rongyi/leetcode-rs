struct Solution;

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let m = 1e9 as i64 + 7;
        let n = n as i64;
        let a = a as i64;
        let b = b as i64;

        let lcm = a * b / Self::gcd(a, b);
        let mut l = 2;
        let mut r = 1e14 as i64;
        while l < r {
            let mid = l + (r - l) / 2;
            if mid / a + mid / b - m / lcm < n as i64 {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        (l % m) as i32
    }

    fn gcd(mut x: i64, mut y: i64) -> i64 {
        let mut r;
        while y > 0 {
            r = x % y;
            x = y;
            y = r;
        }

        x as i64
    }
}

fn main() {}
