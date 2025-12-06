struct Solution;

impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut expected = 0;
        for num in nums.into_iter() {
            if num != expected {
                return expected;
            }
            expected += 1;
        }
        expected
    }
}

fn main() {}
