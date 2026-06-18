struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        let mut acc_from_left = vec![1; sz];
        let mut ret = vec![0; sz];

        let mut acc = nums[0];
        for (i, &v) in nums.iter().enumerate().skip(1) {
            acc_from_left[i] = acc;
            acc *= v;
        }

        let mut acc_from_right = nums[sz - 1];
        ret[sz - 1] = acc_from_left[sz - 1];

        for i in (0..sz - 1).rev() {
            ret[i] = acc_from_left[i] * acc_from_right;
            acc_from_right *= nums[i];
        }

        ret
    }
}

fn main() {}
