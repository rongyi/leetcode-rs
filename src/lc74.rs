struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let sz = m * n;
        let mut low: i32 = 0;
        let mut high: i32 = sz as i32 - 1;

        while low <= high {
            let mid = low + (high - low) / 2;
            let midval = matrix[mid as usize / n][mid as usize % n];

            if target == midval {
                return true;
            } else if target > midval {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        false
    }
}
