struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let total: i32 = (1..=n).sum();
        let mut acc = 0;

        for i in 1..=n {
            let right = total - acc;
            acc += i;

            if acc == right {
                return i;
            }
        }
        -1
    }
}

fn main() {}
