struct Solution;

impl Solution {
    /// LeetCode 688: Knight Probability in Chessboard
    ///
    /// dp[k][r][c] = number of length-k walks on the board that END at (r,c),
    /// where the walk may start at ANY cell. The knight's move graph is
    /// undirected (moves are reversible), so:
    ///   walks(S→any F)  =  walks(any T→S)
    ///
    /// So dp[k][row][col] / 8^k is exactly the probability that a walk
    /// FROM (row, col) stays on the board for k moves.
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let n = n as usize;
        let mut dp = vec![vec![vec![-1.0; n]; n]; (k + 1) as usize];

        Self::recur(&mut dp, n, k, row as usize, column as usize) / 8f64.powf(k as f64)
    }

    fn recur(dp: &mut Vec<Vec<Vec<f64>>>, n: usize, k: i32, r: usize, c: usize) -> f64 {
        if r >= n || c >= n {
            return 0.0; // off board — this position contributes nothing
        }
        if k == 0 {
            return 1.0; // every on-board cell is reachable with 0 moves
        }
        if dp[k as usize][r][c] != -1.0 {
            return dp[k as usize][r][c];
        }

        dp[k as usize][r][c] = 0.0;
        // The 8 knight moves.
        const MOVES: [(i32, i32); 8] = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];
        for (dr, dc) in MOVES {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                dp[k as usize][r][c] += Self::recur(dp, n, k - 1, nr as usize, nc as usize);
            }
        }

        dp[k as usize][r][c]
    }
}

fn main() {
    let tests = [
        (3, 2, 0, 0, 0.0625),
        (1, 0, 0, 0, 1.0),
        (3, 1, 0, 0, 0.25),
        (8, 30, 6, 4, 0.00019052566298311536),
    ];

    for (n, k, row, col, expected) in &tests {
        let result = Solution::knight_probability(*n, *k, *row, *col);
        let pass = (result - expected).abs() < 1e-9;
        println!(
            "{} n={} k={} ({},{}) → {:.12} (expected {:.12})",
            if pass { "✓" } else { "✗" },
            n,
            k,
            row,
            col,
            result,
            expected
        );
    }
}
