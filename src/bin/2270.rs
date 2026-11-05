struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let sum: i64 = nums.iter().sum();
        let sz = nums.len();
        let mut acc = 0i64;
        let mut valid_split = 0;
        for i in 0..sz - 1 {
            acc += nums[i];
            let right = sum - acc;
            if acc >= right {
                valid_split += 1;
            }
        }
        valid_split
    }
}

fn main() {}
