struct Solution;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut count = [0; 101];
        for &num in &nums {
            count[num as usize] += 1;
            if count[num as usize] > 2 {
                return false;
            }
        }
        true
    }
}


fn main() {}
