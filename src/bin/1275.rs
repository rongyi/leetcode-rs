#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut grid = vec![vec![0; 3]; 3];
        for (i, m) in moves.iter().enumerate() {
            if i % 2 == 0 {
                grid[m[0] as usize][m[1] as usize] = 1;
            } else {
                grid[m[0] as usize][m[1] as usize] = -1;
            }
        }

        for r in grid.iter() {
            let sum: i32 = r.iter().sum();
            if sum == 3 {
                return "A".to_string();
            }
            if sum == -3 {
                return "B".to_string();
            }
        }

        for col in 0..3 {
            let sum = grid[0][col] + grid[1][col] + grid[2][col];
            if sum == 3 {
                return "A".to_string();
            }
            if sum == -3 {
                return "B".to_string();
            }
        }
        let diag1 = grid[0][0] + grid[1][1] + grid[2][2];
        let diag2 = grid[0][2] + grid[1][1] + grid[2][0];
        if diag1 == 3 || diag2 == 3 {
            return "A".to_string();
        }
        if diag1 == -3 || diag2 == -3 {
            return "B".to_string();
        }

        if moves.len() < 9 {
            return "Pending".to_string();
        }
        "Draw".to_string()
    }
}

fn main() {}
