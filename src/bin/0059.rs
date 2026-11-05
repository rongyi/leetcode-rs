struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let total = n * n;
        let mut grid = vec![vec![0; n as usize]; n as usize];
        let mut val = 1;

        let mut x = 0;
        let mut y = 0;

        let mut direction = 0;
        let mut right_end = n;
        let mut down_end = n;
        let mut left_end = -1;
        let mut up_end = 0;

        for _ in 0..total {
            grid[x][y] = val;

            match direction {
                0 => {
                    if y + 1 == right_end as usize {
                        direction = 1;
                        x += 1;
                        right_end -= 1;
                    } else {
                        y += 1;
                    }
                }
                1 => {
                    if x + 1 == down_end as usize {
                        direction = 2;
                        y -= 1;
                        down_end -= 1;
                    } else {
                        x += 1;
                    }
                }
                2 => {
                    if y as i32 - 1 == left_end {
                        direction = 3;
                        x -= 1;
                        left_end += 1;
                    } else {
                        y -= 1;
                    }
                }
                3 => {
                    if x as i32 - 1 == up_end {
                        y += 1;
                        up_end += 1;
                        direction = 0;
                    } else {
                        x -= 1
                    }
                }
                _ => unreachable!(),
            }

            val += 1;
        }

        grid
    }
}

fn main() {}
