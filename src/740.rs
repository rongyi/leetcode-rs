#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut score = vec![0; 10001];

        for &num in nums.iter() {
            score[num as usize] += num;
        }

        // take[i] = skip[i-1] + values[i];
        // skip[i] = Math.max(skip[i-1], take[i-1]);
        // take[i] can only be derived from: if you skipped the [i-1]th bucket, and you take bucket[i].
        // skip[i] through, can be derived from either take[i-1] or skip[i-1], whatever the bigger;
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
