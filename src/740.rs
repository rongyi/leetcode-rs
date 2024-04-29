#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut score = vec![0; 10001];

        for &num in nums.iter() {
            score[num as usize] += num;
        }

        let mut take = 0;
        let mut skip = 0;
        for &num in score.iter() {
            let takei = skip + num;
            let skipi = skip.max(take);

            take = takei;
            skip = skipi;
        }

        take.max(skip)
    }
}

fn main() {}
