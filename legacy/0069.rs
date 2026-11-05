struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }
        let x: i64 = x as i64;
        let mut left = 1;
        let mut right = x;
        let mut ret = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            let cur = mid * mid;
            if cur == x {
                return mid as i32;
            } else if cur > x {
                right = mid - 1;
            } else {
                ret = mid;
                left = mid + 1;
            }
        }

        ret as i32
    }
}
