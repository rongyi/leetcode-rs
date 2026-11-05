struct Solution;

impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut horizen_acc = vec![vec![0i32; n]; m];
        let mut vertical_acc = vec![vec![0i32; n]; m];
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    horizen_acc[i][j] = if j == 0 { 1 } else { horizen_acc[i][j - 1] + 1 };
                    vertical_acc[i][j] = if i == 0 {
                        1
                    } else {
                        vertical_acc[i - 1][j] + 1
                    };
                }
            }
        }
        let mut max_edge = 0;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let mut cur_edge = horizen_acc[i][j].min(vertical_acc[i][j]);
                while cur_edge > max_edge {
                    if vertical_acc[i][j - cur_edge as usize + 1] >= cur_edge
                        && horizen_acc[i - cur_edge as usize + 1][j] >= cur_edge
                    {
                        max_edge = cur_edge;
                        break;
                    }
                    cur_edge -= 1;
                }
            }
        }

        max_edge * max_edge
    }
}

fn main() {}
