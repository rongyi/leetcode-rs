struct Solution;

impl Solution {
    pub fn search_matrix(grid: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut left = 0i32;
        let mut right = (m * n - 1) as i32;
        while left <= right {
            let mid = left + (right - left) / 2;
            let x = mid / n as i32;
            let y = mid % n as i32;

            if grid[x as usize][y as usize] == target {
                return true;
            } else if grid[x as usize][y as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        false
    }
}

fn main() {}
