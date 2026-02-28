struct Solution;

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        for idx in 0..32 {
            let mut total_one = 0;
            for &num in nums.iter() {
                if num & (1 << idx) != 0 {
                    total_one += 1;
                }
            }
            if total_one >= k {
                ret |= 1 << idx;
            }
        }
        ret
    }
}

fn main() {}
