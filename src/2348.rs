struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let sz = nums.len();
        let mut i = 0;
        let mut total = 0;
        while i < sz {
            if nums[i] == 0 {
                let mut j = i;
                while j < sz && nums[j] == 0 {
                    j += 1;
                }
                let len = (j - i) as i64;
                total += (1..=len).sum::<i64>();
                i = j;
            } else {
                i += 1;
            }
        }
        total
    }
}

fn main() {}
