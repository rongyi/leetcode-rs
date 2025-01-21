#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut diag: HashMap<i32, Vec<i32>> = HashMap::new();

        for i in 0..m {
            for j in 0..n {
                let key = i as i32 - j as i32;
                diag.entry(key).or_default().push(mat[i][j]);
            }
        }
        for (&k, v) in diag.iter_mut() {
            v.sort_unstable();
            let mut idx = 0;
            for i in 0..m {
                let j = i as i32 - k;
                if j < 0 || j >= n as i32 {
                    continue;
                }
                mat[i][j as usize] = v[idx];
                idx += 1;
            }
        }

        mat
    }
}

fn main() {}
