struct Solution;

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let total = nums.len() as i32;
        for i in 0..32 {
            let mut one_count = 0;
            for &num in &nums {
                one_count += (num >> i) & 1;
            }
            ret += one_count * (total - one_count);
        }
        ret
    }
}

fn main() {}
