struct Solution;

impl Solution {
    /// LeetCode 329: Longest Increasing Path in a Matrix
    ///
    /// From each cell, you can move up/down/left/right to a cell with a
    /// strictly larger value. Find the length of the longest such path.
    ///
    /// # Approach: DFS + memoization
    ///
    /// Since values must increase, there are no cycles — DFS is safe.
    /// `memo[x][y]` caches the longest path starting from (x,y).
    /// O(m·n) time and space.
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut memo = vec![vec![0i32; n]; m];
        let mut ans = 0;

        for i in 0..m {
            for j in 0..n {
                ans = ans.max(Self::dfs(&matrix, i, j, &mut memo));
            }
        }

        ans
    }

    fn dfs(matrix: &[Vec<i32>], x: usize, y: usize, memo: &mut [Vec<i32>]) -> i32 {
        if memo[x][y] != 0 {
            return memo[x][y];
        }

        let dirs = [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)];
        let mut best = 1;

        for (dx, dy) in dirs {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && (nx as usize) < matrix.len()
                && ny >= 0
                && (ny as usize) < matrix[0].len()
                && matrix[nx as usize][ny as usize] > matrix[x][y]
            {
                best = best.max(1 + Self::dfs(matrix, nx as usize, ny as usize, memo));
            }
        }

        memo[x][y] = best;
        best
    }
}

fn main() {}
