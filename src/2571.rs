struct Solution;

impl Solution {
    pub fn min_operations(mut n: i32) -> i32 {
        let mut cnt = 0;
        while n > 0 {
            if n & 3 == 3 {
                n += 1;
                cnt += 1;
            } else {
                cnt += n & 1;
                n >>= 1;
            }
        }

        cnt
    }
}

fn main() {}
