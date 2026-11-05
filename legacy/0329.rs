struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let m = matrix.len();
        let n = matrix[0].len();
        let mut path = vec![vec![-1; n]; m];
        let mut ret = 0;

        fn startFromHere(
            matrix: &Vec<Vec<i32>>,
            x: usize,
            y: usize,
            dirs: &[[i32; 2]; 4],
            path: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if path[x][y] != -1 {
                return path[x][y];
            }
            path[x][y] = 1;
            for d in dirs {
                let nx = x as i32 + d[0];
                let ny = y as i32 + d[1];
                if nx < 0
                    || nx >= matrix.len() as i32
                    || ny < 0
                    || ny >= matrix[0].len() as i32
                    || matrix[nx as usize][ny as usize] <= matrix[x as usize][y as usize]
                {
                    continue;
                }
                path[x][y] =
                    path[x][y].max(1 + startFromHere(matrix, nx as usize, ny as usize, dirs, path));
            }
            path[x][y]
        }
        for i in 0..m {
            for j in 0..n {
                if path[i][j] == -1 {
                    ret = ret.max(startFromHere(&matrix, i, j, &dirs, &mut path));
                }
            }
        }

        ret
    }
}

fn main() {}
