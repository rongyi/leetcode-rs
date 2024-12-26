#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut row_acc = vec![0; m];
        let mut col_acc = vec![0; n];

        for p in indices.iter() {
            row_acc[p[0] as usize] += 1;
            col_acc[p[1] as usize] += 1;
        }
        let mut odd_acc = 0;
        for i in 0..m {
            for j in 0..n {
                let val = row_acc[i] + col_acc[j];
                if val % 2 == 1 {
                    odd_acc += 1;
                }
            }
        }

        odd_acc
    }
}

fn main() {}
