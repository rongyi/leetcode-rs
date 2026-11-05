#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut b = [[false; 8]; 8];

        for q in queens.iter() {
            b[q[0] as usize][q[1] as usize] = true;
        }

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let mut nx = king[0] + i;
                let mut ny = king[1] + j;
                while nx.min(ny) >= 0 && nx.max(ny) < 8 {
                    if b[nx as usize][ny as usize] {
                        ret.push(vec![nx, ny]);
                        break;
                    }
                    nx += i;
                    ny += j;
                }
            }
        }

        ret
    }
}

fn main() {}
