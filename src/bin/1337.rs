#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let m = mat.len();
        let row_score: Vec<i32> = mat
            .iter()
            .map(|r| {
                let mut score = 0;
                for &num in r.iter() {
                    if num == 0 {
                        break;
                    }
                    score += 1;
                }
                score
            })
            .collect();
        let mut sorted: Vec<usize> = (0..m).collect();
        sorted.sort_by_key(|&r| row_score[r]);
        sorted.truncate(k as usize);

        sorted.into_iter().map(|a| a as i32).collect()
    }
}

fn main() {}
