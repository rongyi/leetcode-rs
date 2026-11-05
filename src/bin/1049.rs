struct Solution;

fn main() {}

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum: i32 = stones.iter().sum();
        let target = sum as usize / 2;

        let mut dp = vec![0; target + 1];

        for &stone in stones.iter() {
            let weight = stone as usize;
            for j in (weight..=target).rev() {
                // either pick this stone:
                dp[j] = dp[j].max(dp[j - weight] + stone);
            }
        }

        sum - 2 * dp[target] as i32
    }
}
