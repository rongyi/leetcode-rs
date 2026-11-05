#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        if boarding_cost <= 0 && running_cost >= 0 {
            return -1;
        }

        let mut waiting = 0;
        let mut total_profit = 0;
        let mut max_profit = 0;
        let mut rotation = 0;
        let mut max_profit_rotation = -1;

        let mut i = 0;
        while i < customers.len() || waiting > 0 {
            // Add new customers to waiting line
            if i < customers.len() {
                waiting += customers[i];
            }

            // Board customers
            let board = waiting.min(4);
            waiting -= board;

            // Calculate profit
            let profit = board as i32 * boarding_cost - running_cost;
            total_profit += profit;

            rotation += 1;

            // Update max profit
            if total_profit > max_profit {
                max_profit = total_profit;
                max_profit_rotation = rotation;
            }

            i += 1;
        }

        max_profit_rotation
    }
}
fn main() {}
