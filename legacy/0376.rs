struct Solution;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        if sz <= 1 {
            return sz as i32;
        }
        let mut reduce = Vec::new();
        for i in 1..sz {
            if nums[i] - nums[i - 1] > 0 {
                reduce.push(1);
            } else if nums[i] - nums[i -1] < 0 {
                reduce.push(-1);
            }
        }
        let mut ret = 1;
        for i in 0..reduce.len() {
            if i == 0 || reduce[i] != reduce[i - 1] {
                ret += 1;
            }
        }
        ret
    }
}


fn main() {}
