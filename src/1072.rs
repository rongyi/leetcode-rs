struct Solution;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 0..m {
            let mut flip = Vec::with_capacity(n);
            for &num in matrix[i].iter() {
                flip.push(1 - num);
            }
            let mut cur = 0;
            for k in i..m {
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
