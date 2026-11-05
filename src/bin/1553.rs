#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_days(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Self::dp(n, &mut memo)
    }

    fn dp(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n <= 1 {
            return n;
        }

        if let Some(&days) = memo.get(&n) {
            return days;
        }

        // 1. We define a recursive function `dp(n)` that returns the minimum days needed to eat n oranges.
        // 2. The base case is when n = 0 or n = 1, where the answer is simply n.
        // 3. For each state, we have three options:
        //    - Eat one orange and solve for n-1
        //    - Eat n%2 oranges to make n divisible by 2, then eat n/2 oranges
        //    - Eat n%3 oranges to make n divisible by 3, then eat 2n/3 oranges
        // 4. We take the minimum of these three options.
        // 5. To avoid redundant calculations, we use memoization to store results for each n.

        // Try eating n/2 oranges (if possible) and remaining to get to a multiple of 2
        let option1 = 1 + n % 2 + Self::dp(n / 2, memo);

        // Try eating 2n/3 oranges (if possible) and remaining to get to a multiple of 3
        let option2 = 1 + n % 3 + Self::dp(n / 3, memo);

        let result = std::cmp::min(option1, option2);

        memo.insert(n, result);
        return result;
    }
}

fn main() {}
