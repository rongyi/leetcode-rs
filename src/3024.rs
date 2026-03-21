struct Solution;

impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        nums.sort_unstable();
        let (a, b, c) = (nums[0], nums[1], nums[2]);
        if a + b <= c {
            return "none".to_string();
        }
        if a == b && b == c {
            return "equilateral".to_string();
        }
        if a == b || b == c {
            return "isosceles".to_string();
        }

        return "scalene".to_string();
    }
}

fn main() {}
