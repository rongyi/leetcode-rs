struct Solution;

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        z == 0 || z as i64 <= x as i64 + y as i64 && z % Self::gcd(x, y) == 0
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
