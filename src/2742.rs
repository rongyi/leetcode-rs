struct Solution;

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let sz = cost.len();
        let mut dp = vec![vec![0; 501]; 501];
        Self::recur(&cost, &time, 0, sz as i32, &mut dp)
    }

    fn recur(
        cost: &[i32],
        time: &[i32],
        cur: usize,
        wall_left: i32,
        dp: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if wall_left <= 0 {
            return 0;
        }
        if cur >= cost.len() {
            return 1_000_000_000;
        }
        if dp[cur][wall_left as usize] != 0 {
            return dp[cur][wall_left as usize];
        }
        let take = cost[cur] + Self::recur(cost, time, cur + 1, wall_left - time[cur] - 1, dp);
        let not_take = Self::recur(cost, time, cur + 1, wall_left, dp);

        dp[cur][wall_left as usize] = take.min(not_take);
        dp[cur][wall_left as usize]
    }
}

fn main() {}
