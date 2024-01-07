struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_prod = nums[0];
        let mut min_prod = nums[0];
        let mut ret = nums[0];

        for i in 1..nums.len() {
            let tmp: i32 = vec![nums[i], nums[i] * max_prod, nums[i] * min_prod]
                .iter()
                .max()
                .unwrap()
                .to_owned();
            min_prod = vec![nums[i], nums[i] * max_prod, nums[i] * min_prod]
                .iter()
                .min()
                .unwrap()
                .to_owned();

            max_prod = tmp;

            ret = ret.max(max_prod);
        }

        ret
    }
}

fn main() {}
