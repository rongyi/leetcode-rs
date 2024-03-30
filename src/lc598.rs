struct Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut x = m;
        let mut y = n;
        for r in ops.iter() {
            x = x.min(r[0]);
            y = y.min(r[1]);
        }
        x * y
    }
}

fn main() {}
