struct Solution;

impl Solution {
    /// LeetCode 547: Number of Provinces
    ///
    /// Count connected components in an undirected graph given as an
    /// adjacency matrix. DFS from each unvisited city.
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n];
        let mut provinces = 0;

        for city in 0..n {
            if !visited[city] {
                provinces += 1;
                Self::dfs(&is_connected, &mut visited, city);
            }
        }

        provinces
    }

    fn dfs(matrix: &[Vec<i32>], visited: &mut [bool], from: usize) {
        visited[from] = true;
        for to in 0..matrix.len() {
            if matrix[from][to] == 1 && !visited[to] {
                Self::dfs(matrix, visited, to);
            }
        }
    }
}

fn main() {
    let tests = [
        (vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]], 2),
        (vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]], 3),
    ];

    for (matrix, expected) in &tests {
        let result = Solution::find_circle_num(matrix.clone());
        println!(
            "{} is_connected={:?} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            matrix,
            result,
            expected
        );
    }
}
