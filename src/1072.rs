struct Solution;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 0..m {
            // find same pattern, i.e.
            // [1, 0, 1] with [1, 0, 1] will keep same for toggle column 1 with same value
            // [1, 0, 1] with [0, 1, 0] will keep same for toggle colmn 1 , with complement value
            let mut flip = Vec::with_capacity(n);
            for &num in matrix[i].iter() {
                flip.push(1 - num);
            }
            let mut cur = 1;
            for k in i + 1..m {
                if matrix[k].eq(&matrix[i]) || matrix[k].eq(&flip) {
                    cur += 1;
                }
            }
            ret = ret.max(cur);
        }
        ret
    }
}

fn main() {}
