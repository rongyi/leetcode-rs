pub struct Solution;

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let sz = nums.len();
        let mut result = vec![vec![]; sz];
        let mut ret = vec![];

        for i in 0..sz {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && result[j].len() > result[i].len() {
                    result[i] = result[j].clone();
                }
            }
            result[i].push(nums[i]);
            if result[i].len() > ret.len() {
                ret = result[i].clone();
            }
        }

        ret
    }
}

fn main() {}
