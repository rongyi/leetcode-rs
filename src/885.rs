struct Solution;

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let mut cur_dir = 0;
        let mut cur_len = 0;

        let mut x = r_start;
        let mut y = c_start;
        let mut ret = vec![vec![r_start, c_start]];
        let total = (rows * cols) as usize;
        while ret.len() < total {
            if cur_dir == 0 || cur_dir == 2 {
                cur_len += 1;
            }

            for _ in 0..cur_len {
                x += dirs[cur_dir][0];
                y += dirs[cur_dir][1];
                if x >= 0 && x < rows && y >= 0 && y < cols {
                    ret.push(vec![x, y]);
                }
            }

            cur_dir = (cur_dir + 1) % 4;
        }

        ret
    }
}

fn main() {}
