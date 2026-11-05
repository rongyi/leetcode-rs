struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut ret = 0;
        while x != 0 {
            let d = x % 10;
            if ret > i32::MAX / 10 || (ret == i32::MAX / 10 && d > i32::MAX % 10) {
                return 0;
            }
            if ret < i32::MIN / 10 || (ret == i32::MIN / 10 && d < i32::MIN % 10) {
                return 0;
            }
            ret = ret * 10 + d;
            x /= 10;
        }

        ret
    }
}
