struct Solution;

impl Solution {
    /// LeetCode 417: Pacific Atlantic Water Flow
    ///
    /// Return cells that can flow to BOTH Pacific (top/left) and
    /// Atlantic (bottom/right).
    ///
    /// # Approach: reverse DFS from oceans
    ///
    /// Instead of starting DFS from every cell, start from the borders
    /// and work uphill. This visits each cell once per ocean → O(m·n).
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (heights.len(), heights[0].len());

        // reachable[i][j] bit 0 = pacific, bit 1 = atlantic
        let mut reachable = vec![vec![0u8; n]; m];

        // DFS from Pacific borders: top row + left column
        for i in 0..m {
            Self::dfs(&heights, i, 0, -1, &mut reachable, 1);
        }
        for j in 0..n {
            Self::dfs(&heights, 0, j, -1, &mut reachable, 1);
        }

        // DFS from Atlantic borders: bottom row + right column
        for i in 0..m {
            Self::dfs(&heights, i, n - 1, -1, &mut reachable, 2);
        }
        for j in 0..n {
            Self::dfs(&heights, m - 1, j, -1, &mut reachable, 2);
        }

        // Collect cells reachable from both.
        let mut result = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if reachable[i][j] == 3 {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    }

    /// DFS from (x, y) uphill. `prev` is the height we came from.
    /// `mask` = 1 for Pacific, 2 for Atlantic.
    fn dfs(
        heights: &[Vec<i32>],
        x: usize,
        y: usize,
        prev: i32,
        reachable: &mut [Vec<u8>],
        mask: u8,
    ) {
        if reachable[x][y] & mask != 0 {
            return; // already visited from this ocean
        }
        if heights[x][y] < prev {
            return; // can't flow uphill
        }

        reachable[x][y] |= mask;

        let dirs = [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)];
        let (m, n) = (heights.len() as i32, heights[0].len() as i32);

        for (dx, dy) in dirs {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < m && ny >= 0 && ny < n {
                Self::dfs(
                    heights,
                    nx as usize,
                    ny as usize,
                    heights[x][y],
                    reachable,
                    mask,
                );
            }
        }
    }
}

fn main() {
    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    let result = Solution::pacific_atlantic(heights);
    println!("Cells reachable from both oceans: {:?}", result);
}
