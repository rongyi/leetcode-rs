struct Solution;

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let sum1 = nums.iter().sum::<i32>();
        let sum2: i32 = nums
            .into_iter()
            .map(|mut v| {
                let mut sum = 0;
                while v > 0 {
                    sum += v % 10;
                    v /= 10;
                }
                sum
            })
            .sum();

        (sum1 - sum2).abs()
    }
}

fn main() {}
