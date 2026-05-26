struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let w: Vec<char> = word.chars().collect();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::dfs(&mut board, i as i32, j as i32, &w, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, x: i32, y: i32, word: &Vec<char>, pos: usize) -> bool {
        if pos == word.len() {
            return true;
        }
        if x < 0 || x >= board.len() as i32 || y < 0 || y >= board[0].len() as i32 {
            return false;
        }
        if board[x as usize][y as usize] != word[pos] {
            return false;
        }

        let tmp = board[x as usize][y as usize];
        board[x as usize][y as usize] = '#';
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter() {
            let (nx, ny) = (x + dx, y + dy);
            let v = Self::dfs(board, nx, ny, word, pos + 1);
            if v {
                return v;
            }
        }

        board[x as usize][y as usize] = tmp;

        false
    }
}
fn main() {}
