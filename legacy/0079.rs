struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        let m = board.len();
        let n = board[0].len();

        let mut visited = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                if Self::dfs(i, j, &mut visited, &board, &word, 0) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        i: usize,
        j: usize,
        visited: &mut Vec<Vec<bool>>,
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        index: usize,
    ) -> bool {
        if index == word.len() {
            return true;
        }
        if i >= board.len() || j >= board[0].len() || visited[i][j] {
            return false;
        }
        if board[i][j] != word[index] {
            return false;
        }
        visited[i][j] = true;

        let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (x, y) in dirs {
            let nx = i as i32 + x;
            let ny = j as i32 + y;
            if Self::dfs(nx as usize, ny as usize, visited, board, word, index + 1) {
                return true;
            }
        }
        visited[i][j] = false;

        false
    }
}

