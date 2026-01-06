struct Solution;

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut ret = 0;
        // get all the index and then fetch val
        for i in 1..=sz {
            if sz % i == 0 {
                let val = nums[i - 1];
                ret += val * val;
            }
        }
        ret
    }
}

fn main() {}
