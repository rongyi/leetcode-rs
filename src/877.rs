struct Solution;

mod ai {
    struct Solution;

    use std::collections::HashMap;

    impl Solution {
        pub fn stone_game(piles: Vec<i32>) -> bool {
            let n = piles.len();
            let mut memo = HashMap::new();

            Self::solve(&piles, 0, n - 1, &mut memo) > 0
        }

        fn solve(
            piles: &[i32],
            left: usize,
            right: usize,
            memo: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if left > right {
                return 0;
            }

            if left == right {
                return piles[left];
            }

            if let Some(&result) = memo.get(&(left, right)) {
                return result;
            }

            // Current player's advantage = max(choose left, choose right)
            let take_left = piles[left] - Self::solve(piles, left + 1, right, memo);
            let take_right = piles[right] - Self::solve(piles, left, right - 1, memo);
            let result = take_left.max(take_right);

            memo.insert((left, right), result);
            result
        }
    }
}

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();

        // dp[i][j] = maximum stones the current player can get from piles[i..=j]
        // when both play optimally
        let mut dp = vec![vec![0; n]; n];

        // Base case: single pile
        for i in 0..n {
            dp[i][i] = piles[i];
        }

        // Build DP for all intervals
        for length in 2..=n {
            for i in 0..=n - length {
                let j = i + length - 1;
                // Current player can take either piles[i] or piles[j]
                // Then the opponent gets the optimal from the remaining piles
                dp[i][j] = (piles[i] - dp[i + 1][j]).max(piles[j] - dp[i][j - 1]);
            }
        }

        // Alex wins if his advantage is > 0
        dp[0][n - 1] > 0
    }
}

fn main() {}
