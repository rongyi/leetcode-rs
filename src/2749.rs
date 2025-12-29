struct Solution;

impl Solution {
    pub fn make_the_integer_zero(mut num1: i32, num2: i32) -> i32 {
        let mut num1: i64 = num1 as i64;
        let num2: i64 = num2 as i64;
        let mut cnt = 1;
        while cnt < 35 && num1 > 0 {
            num1 -= num2;

            if num1.count_ones() as i64 <= cnt && cnt <= num1 {
                return cnt as _;
            }

            cnt += 1;
        }
        -1
    }
}
fn main() {}
