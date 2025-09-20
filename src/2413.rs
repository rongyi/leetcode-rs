struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n <= 1 {
            return 2;
        }
        if n % 2 == 0 {
            return n;
        }
        n * 2
    }
}

fn main() {}
