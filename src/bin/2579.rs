struct Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        if n == 1 {
            return 1;
        }
        let mut prev = 1;
        let mut cur: i64 = 0;
        for i in 2..=n as i64 {
            cur = prev + (i - 1) * 4;
            prev = cur;
        }

        cur
    }
}
fn main() {}
