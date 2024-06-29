#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        // simulation
        let mut row = vec![poured as f64];
        for r in 0..query_row as usize {
            let mut next_row = vec![0.0; r + 2];
            for i in 0..=r {
                if row[i] > 1.0 {
                    next_row[i] += (row[i] - 1.0) / 2.0;
                    next_row[i + 1] += (row[i] - 1.0) / 2.0;
                }
            }

            row = next_row;
        }

        row[query_glass as usize].min(1.0)
    }
}

fn main() {
    let val = Solution::champagne_tower(2, 1, 1);
    println!("{}", val);
}
