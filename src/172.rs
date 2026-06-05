struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut cnt = 0;
        while n > 0 {
            n /= 5;
            cnt += n;
        }

        cnt
    }
}

fn main() {}
