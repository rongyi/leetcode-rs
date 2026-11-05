struct Solution;

impl Solution {
    pub fn make_integer_beautiful(mut n: i64, target: i32) -> i64 {
        let mut ret = 0;
        let mut base = 1;
        let target = target as i64;

        while Self::sum_digit(n) > target {
            // the heart line
            ret += base * 10 - (n % 10) * base;

            n /= 10;
            n += 1;
            base *= 10;
        }

        ret
    }
    fn sum_digit(mut n: i64) -> i64 {
        let mut acc = 0;
        while n > 0 {
            acc += n % 10;
            n /= 10;
        }
        acc
    }
}

fn main() {}
