struct Solution;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as i64;
        let mut left = 0;
        let mut right = (c as f64).sqrt() as i64;
        while left <= right {
            let cur = left * left + right * right;
            if cur == c {
                return true;
            } else if cur < c {
                left += 1;
            } else {
                right -= 1;
            }
        }
        false
    }
}

fn main() {}
