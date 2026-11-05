struct Solution;

impl Solution {
    pub fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
        let sz = nums.len();

        if sz <= 2 {
            return true;
        }

        nums.windows(2).any(|pair| pair[0] + pair[1] >= m)
    }
}

fn main() {}
