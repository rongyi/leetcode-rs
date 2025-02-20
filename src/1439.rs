#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut cur_sum = mat[0].clone();
        for i in 1..mat.len() {
            let mut next_sum = vec![];

            for j in 0..mat[i].len() {
                for &path in cur_sum.iter() {
                    next_sum.push(path + mat[i][j]);
                }
            }
            next_sum.sort_unstable();
            next_sum.truncate((k as usize).min(next_sum.len()));

            cur_sum = next_sum;
        }

        cur_sum[k as usize - 1]
    }
}

fn main() {}
