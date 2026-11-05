struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut val = 0;

        for i in 0..32 {
            let mut acc = 0;
            for &v in nums.iter() {
                acc += (v >> i) & 1;
            }
            acc %= 3;
            val |= acc << i;
        }

        val
    }
}
fn main() {}
