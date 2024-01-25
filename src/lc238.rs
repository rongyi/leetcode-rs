struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        let mut prefix = vec![1; sz];
        let mut suffix = vec![1; sz];
        let mut ret = vec![0; sz];

        for i in 1..sz {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }
        for i in (0..sz - 1).rev() {
            suffix[i] = suffix[i + 1] * nums[i + 1];
        }

        for i in 0..sz {
            ret[i] = prefix[i] * suffix[i];
        }

        ret
    }
}

fn main() {}
