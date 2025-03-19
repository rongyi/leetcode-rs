#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut inventory = inventory;
        let mut orders = orders as i64;
        let mut result: i64 = 0;

        // Sort inventory in descending order
        inventory.sort_unstable_by(|a, b| b.cmp(a));
        inventory.push(0); // Add sentinel value for easier processing

        let mut count = 1i64; // Count of colors with the same value

        for i in 0..inventory.len() - 1 {
            let current_value = inventory[i] as i64;
            let next_value = inventory[i + 1] as i64;
            let diff = current_value - next_value;

            if diff == 0 {
                // Same value as the next one, just increment count
                count += 1;
                continue;
            }

            // Total balls we can sell in this value range
            let balls_in_range = count * diff;

            if balls_in_range <= orders {
                // We can sell all balls in this value range
                // For each ball color, we sell balls with values: current_value, current_value-1, ..., next_value+1
                // Using sum formula: n/2 * (first + last) = n/2 * (current_value + (next_value+1))
                let sum = count * (current_value + next_value + 1) * diff / 2;
                result = (result + sum) % MOD;
                orders -= balls_in_range;
            } else {
                // // This branch handles the case where we can't sell all the balls in the current range.
                // For example, if we have inventory [5,5,5] and orders = 7, we can't sell all balls down to the next level (0).
                //
                // complete_rows: How many levels we can fully complete across all colors
                // If count = 3 (colors) and orders = 7, then complete_rows = 7/3 = 2
                // This means we can sell 2 balls from each color (values 5,4)
                //
                // remainder: How many additional balls we need after taking complete_rows from each color
                // In our example, remainder = 7%3 = 1, so after taking 2 from each color (6 total),
                // we need 1 more ball, which we take at value 3
                // We can only sell a portion of the balls
                let complete_rows = orders / count; // How many balls we can take from each color
                let remainder = orders % count; // Leftover balls needed

                if complete_rows > 0 {
                    // Process complete rows
                    let last_value = current_value - complete_rows + 1;
                    let sum = count * (current_value + last_value) * complete_rows / 2;
                    result = (result + sum) % MOD;
                }

                // Process remainder - all have the same value: current_value - complete_rows
                let remaining_value = current_value - complete_rows;

                // We multiply by remainder because we're selling 'remainder' number of balls
                // each with the same value (remaining_value). For example, if remainder=3 and
                // remaining_value=5, we sell 3 balls at value 5, giving 3*5=15 profit.
                // The math here is brilliant - we calculated EXACTLY how many balls of each value to take
                // without having to simulate the process ball-by-ball. This is much more efficient!
                result = (result + remainder * remaining_value) % MOD;

                orders = 0; // We've fulfilled all orders
            }

            count += 1; // Move to next color

            if orders == 0 {
                break; // Early termination
            }
        }

        result as i32
    }
}
fn main() {}
