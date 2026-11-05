struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }
        let x = x as i64;
        let mut left = 1;
        let mut right = x as i64;
        while left <= right {
            let mid = left + (right - left) / 2;
            let try_square = mid * mid;
            if try_square == x {
                return mid as i32;
            } else if try_square > x {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        right as i32
    }
}

fn main() {}
