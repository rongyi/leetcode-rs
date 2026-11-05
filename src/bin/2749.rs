struct Solution;

impl Solution {
    pub fn make_the_integer_zero(mut num1: i32, num2: i32) -> i32 {
        let mut num1: i64 = num1 as i64;
        let num2: i64 = num2 as i64;
        let mut cnt = 1;
        // With cnt operations, num1 should equal: num1 == cnt * num2 + diff.

        // diff is the sum of cnt single-bit numbers; a single-bit number is 2 power
        // something.

        // So, we check if we can make diff:

        // 1. Number of bits in diff should not exceed cnt.
        // 2. The minimum possible sum(which we call diff) is cnt * (2 ^ 0), so cnt
        // should not be larger than diff.  i.e. diff >= cnt

        while cnt < 35 && num1 > 0 {
            num1 -= num2;

            // here num1 is diff
            if num1.count_ones() as i64 <= cnt && cnt <= num1 {
                return cnt as _;
            }

            cnt += 1;
        }
        -1
    }
}
fn main() {}
