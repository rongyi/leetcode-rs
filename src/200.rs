struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut ret = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    ret += 1;
                    Self::visit(&mut grid, i as i32, j as i32);
                }
            }
        }

        ret
    }

    fn visit(grid: &mut Vec<Vec<char>>, x: i32, y: i32) {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        if x < 0 || x >= m || y < 0 || y >= n || grid[x as usize][y as usize] != '1' {
            return;
        }
        grid[x as usize][y as usize] = '0';

        Self::visit(grid, x + 1, y);
        Self::visit(grid, x - 1, y);
        Self::visit(grid, x, y + 1);
        Self::visit(grid, x, y - 1);
    }
}

fn main() {}
