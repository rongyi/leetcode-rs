struct Solution;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut left = 1;
        let mut right = cells.len() as i32;
        let mut res = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::can_cross(row, col, &cells, mid) {
                res = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        res
    }

    fn can_cross(row: i32, col: i32, cells: &Vec<Vec<i32>>, day: i32) -> bool {
        let mut grid = vec![vec![0; col as usize]; row as usize];
        for i in 0..day as usize {
            let r = cells[i][0] - 1;
            let c = cells[i][1] - 1;
            grid[r as usize][c as usize] = 1;
        }

        let mut queue = std::collections::VecDeque::new();
        for c in 0..col {
            if grid[0][c as usize] == 0 {
                queue.push_back((0, c));
                grid[0][c as usize] = 1;
            }
        }

        let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        while let Some((r, c)) = queue.pop_front() {
            if r == row - 1 {
                return true;
            }
            for (dr, dc) in dirs.iter() {
                let nr = r + dr;
                let nc = c + dc;
                if nr >= 0 && nr < row && nc >= 0 && nc < col && grid[nr as usize][nc as usize] == 0
                {
                    grid[nr as usize][nc as usize] = 1;
                    queue.push_back((nr, nc));
                }
            }
        }

        false
    }
}

fn main() {}
