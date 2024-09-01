struct Solution;

impl Solution {
    pub fn prime_palindrome(mut n: i32) -> i32 {
        let mut n = n as i64;
        if n >= 1e7 as i64 && n <= 1e8 as i64 {
            n = 100030001;
        }
        loop {
            let revn = Self::create_palindrom(n);
            if revn != n {
                n += 1;
                continue;
            }
            if Self::is_prime(n) {
                return n as i32;
            }
            n += 1;
        }
    }

    fn create_palindrom(mut n: i64) -> i64 {
        let mut ret = 0;

        while n > 0 {
            ret = ret * 10 + n % 10;
            n /= 10;
        }

        ret
    }
    fn is_prime(n: i64) -> bool {
        if n < 2 {
            return false;
        }
        let upper = (n as f64).sqrt() as i64;
        for i in 2..=upper {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {}
