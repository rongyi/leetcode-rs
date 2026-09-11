struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let last_day = *days.last().unwrap() as usize;

        // Mark which calendar days are travel days
        let mut is_travel_day = vec![false; last_day + 1];
        for &day in &days {
            is_travel_day[day as usize] = true;
        }

        // dp[i] = minimum cost to cover all travel days up to day i
        let mut dp = vec![0; last_day + 1];

        for i in 1..=last_day {
            if !is_travel_day[i] {
                // Not traveling today, cost remains same as yesterday
                dp[i] = dp[i - 1];
            } else {
                // Consider buying 1-day, 7-day, or 30-day pass today
                let cost1 = dp[i - 1] + costs[0];
                let cost7 = dp[i.saturating_sub(7)] + costs[1];
                let cost30 = dp[i.saturating_sub(30)] + costs[2];

                dp[i] = cost1.min(cost7).min(cost30);
            }
        }

        dp[last_day]
    }
}

fn main() {}
