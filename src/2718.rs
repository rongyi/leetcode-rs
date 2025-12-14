struct Solution;

impl Solution {
    pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut col_shadowed = vec![false; n];
        let mut row_shadowed = vec![false; n];
        let mut shadow_row_acc = 0;
        let mut shadow_col_acc = 0;
        let mut ret = 0;

        for q in queries.into_iter().rev() {
            let (overwrite_type, index, val) = (q[0], q[1] as usize, q[2] as i64);
            // for row
            if overwrite_type == 0 && !row_shadowed[index] {
                row_shadowed[index] = true;
                shadow_row_acc += 1;
                ret += (n as i64 - shadow_col_acc) * val;
            }

            if overwrite_type == 1 && !col_shadowed[index] {
                col_shadowed[index] = true;
                shadow_col_acc += 1;
                ret += (n as i64 - shadow_row_acc) * val;
            }
        }

        ret
    }
}

fn main() {}
