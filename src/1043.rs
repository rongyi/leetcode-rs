struct Solution;

impl Solution {
    pub fn max_sum_after_partitioning(nums: Vec<i32>, chunk: i32) -> i32 {
        let sz = nums.len();
        let mut dp = vec![0; sz];

        for i in 0..sz {
            let mut cur_max = 0;
            for l in 1..=chunk {
                // the first index in chunk which len is l
                if i as i32 - l + 1 < 0 {
                    break;
                }
                cur_max = cur_max.max(nums[(i as i32 - l + 1) as usize]);
                dp[i] = dp[i].max(cur_max * l + if i as i32 >= l { dp[i - l as usize] } else { 0 });
            }
        }

        dp[sz - 1]
    }
}

fn main() {}
