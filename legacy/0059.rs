struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![0; n as usize]; n as usize];
        // the start index
        let mut count = 1;
        let mut left = 0;
        let mut right = n - 1;
        let mut top = 0;
        let mut bottom = n - 1;

        while left <= right && top <= bottom {
            for c in left..=right {
                ret[top as usize][c as usize] = count;
                count += 1;
            }
            top += 1;

            for r in top..=bottom {
                ret[r as usize][right as usize] = count;
                count += 1;
            }

            right -= 1;

            if top <= bottom {
                for c in (left..=right).rev() {
                    ret[bottom as usize][c as usize] = count;
                    count += 1;
                }
                bottom -= 1;
            }

            if left <= right {
                for r in (top..=bottom).rev() {
                    ret[r as usize][left as usize] = count;
                    count += 1;
                }
                left += 1;
            }
        }

        ret
    }
}
