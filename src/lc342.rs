struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        if n & 1 != 0 {
            return n == 1;
        }
        if n % 4 != 0 {
            return false;
        }
        return Self::is_power_of_four(n / 4)
    }
}

fn main() {}
