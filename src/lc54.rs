struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![];
        let m = matrix.len();
        let n = matrix[0].len();

        let mut top: i32 = 0;
        let mut right: i32 = (n - 1) as i32;
        let mut bottom: i32 = (m - 1) as i32;
        let mut left: i32 = 0;

        while top <= bottom && left <= right {
            // top line from left to right
            // from left to right
            for c in left..=right {
                ret.push(matrix[top as usize][c as usize]);
            }
            top += 1;

            // from top to bottom(right side dow)
            for r in top..=bottom {
                ret.push(matrix[r as usize][right as usize]);
            }
            right -= 1;

            // bottom left -> right
            if top <= bottom {
                for c in (left..=right).rev() {
                    ret.push(matrix[bottom as usize][c as usize]);
                }
                bottom -= 1;
            }

            if left <= right {
                for r in (top..=bottom).rev() {
                    ret.push(matrix[r as usize][left as usize]);
                }

                left += 1;
            }
        }

        ret
    }
}
