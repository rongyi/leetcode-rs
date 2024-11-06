struct Solution;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum: i32 = stones.iter().sum();
        let sz = sum / 2 + 1;
        let mut dp = vec![false; sz as usize];
        dp[0] = true;
        let mut max_sum = 0;
        for stone in stones {
            for j in (stone..sz).rev() {
                if dp[(j - stone) as usize] {
                    dp[j as usize] = true;
                    max_sum = max_sum.max(j);
                }
            }
        }

        sum - 2 * max_sum
    }
}

fn main() {}
