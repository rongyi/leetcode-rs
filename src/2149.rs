struct Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let postive_nums: Vec<i32> = nums.clone().into_iter().filter(|x| *x > 0).collect();
        let negtive_nums: Vec<i32> = nums.clone().into_iter().filter(|x| *x < 0).collect();
        if postive_nums.len() != negtive_nums.len() {
            return vec![];
        }

        let mut ret = vec![];

        for i in 0..postive_nums.len() {
            ret.push(postive_nums[i]);
            ret.push(negtive_nums[i]);
        }

        ret
    }
}

fn main() {}
