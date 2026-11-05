struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let sz = matrix.len();
        let mut sum: i64 = 0;
        let mut min_num = i64::MAX;

        let mut neg_count = 0;
        for i in 0..sz {
            for j in 0..sz {
                if matrix[i][j] < 0 {
                    neg_count += 1;
                }
                sum += matrix[i][j].abs() as i64;
                min_num = min_num.min(matrix[i][j].abs() as i64)
            }
        }
        if neg_count % 2 == 1 {
            sum -= min_num * 2;
        }
        sum
    }
}

fn main() {}
