pub struct Solution;

impl Solution {
    pub fn is_power_of_four(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        while n != 1 {
            if n % 4 != 0 {
                return false;
            }
            n /= 4;
        }
        n == 1
    }
}

fn main() {}
