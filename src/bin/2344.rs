struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let mut g = nums_divide[0];

        for &n in nums_divide.iter() {
            g = Self::gcd(g, n);
        }
        nums.sort_unstable();
        for (i, &v) in nums.iter().enumerate() {
            if g % v == 0 {
                return i as _;
            }
        }

        -1
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
