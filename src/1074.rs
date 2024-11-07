
struct Solution;

use std::collections::HashMap;
impl Solution {
    // For each row, calculate the prefix sum.
    // For each pair of columns,
    // calculate the accumulated sum of rows.
    // Now this problem is same to, "Find the Subarray with Target Sum".
    pub fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 0..m {
            for j in 1..n {
                matrix[i][j] += matrix[i][j - 1];
            }
        }
        let mut ret = 0;
        // i, j is two column
        for i in 0..n {
            for j in i..n {
                let mut cnt = HashMap::new();
                cnt.insert(0, 1);
                let mut cur_sum = 0;
                for k in 0..m {
                    cur_sum += matrix[k][j] - if i > 0 { matrix[k][i - 1] } else { 0 };
                    ret += cnt.get(&(cur_sum - target)).unwrap_or(&0);
                    *cnt.entry(cur_sum).or_insert(0) += 1;
                }
            }
        }
        ret
    }
}

fn main() {}
