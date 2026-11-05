struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut total = m * n;
        let mut cur_x = 0;
        let mut cur_y = 0;
        let mut ret = vec![];
        // 0:  right
        // 1:  down
        // 2:  left
        // 3:  up
        let mut direction = 0;

        let mut right_end = n as i32;
        let mut down_end = m as i32;
        let mut left_end = -1 as i32;
        let mut up_end = -1 as i32;
        for _ in 0..total {
            ret.push(matrix[cur_x][cur_y]);
            match direction {
                0 => {
                    // try add
                    if cur_y as i32 + 1 == right_end {
                        up_end += 1;
                        cur_x += 1;
                        direction = 1;
                    } else {
                        cur_y += 1;
                    }
                }
                1 => {
                    if cur_x as i32 + 1 == down_end {
                        right_end -= 1;
                        cur_y -= 1;
                        direction = 2;
                    } else {
                        cur_x += 1;
                    }
                }
                2 => {
                    if cur_y as i32 - 1 == left_end {
                        down_end -= 1;
                        cur_x -= 1;
                        direction = 3;
                    } else {
                        cur_y -= 1;
                    }
                }
                3 => {
                    if cur_x as i32 - 1 == up_end {
                        left_end += 1;
                        direction = 0;
                        cur_y += 1;
                    } else {
                        cur_x -= 1;
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        }

        ret
    }
}
fn main() {}
