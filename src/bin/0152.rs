struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ret = nums[0];
        let mut max_acc = nums[0];
        let mut min_acc = nums[0];

        for &v in nums.iter().skip(1) {
            if v < 0 {
                std::mem::swap(&mut max_acc, &mut min_acc);
            }
            max_acc = std::cmp::max(v, max_acc * v);
            min_acc = std::cmp::min(v, min_acc * v);
            ret = ret.max(max_acc);
        }

        ret
    }
}

fn main() {}
