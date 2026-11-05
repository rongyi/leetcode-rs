struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let a = *nums.iter().min().unwrap();
        let b = *nums.iter().max().unwrap();
        Self::gcd(a, b)
    }
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

fn main() {}
