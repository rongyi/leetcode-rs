struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, mut k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut pq = BinaryHeap::new();
        pq.push(Reverse((matrix[0][0], 0)));
        let mut ret = 0;
        while k != 0 {
            let Reverse((val, pos)) = pq.pop().unwrap();
            ret = val;
            let r = pos / n;
            let c = pos % n;
            let nextr = r + 1;
            let nextc = c + 1;

            // dedup, when when c == 0 we go down
            if c == 0 && nextr < m {
                pq.push(Reverse((matrix[nextr][c], nextr * n + c)));
            }
            // always add right
            if nextc < n {
                pq.push(Reverse((matrix[r][nextc], r * n + nextc)));
            }

            k -= 1;
        }

        ret
    }
}

fn main() {}
