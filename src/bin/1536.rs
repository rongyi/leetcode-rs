#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut trailing_zeros = vec![0; n];

        // Count trailing zeros for each row
        for i in 0..n {
            let mut zeros = 0;
            for j in (0..n).rev() {
                if grid[i][j] == 0 {
                    zeros += 1;
                } else {
                    break;
                }
            }
            trailing_zeros[i] = zeros;
        }

        let mut swaps = 0;

        // For each row, find a suitable row to swap with
        for i in 0..n {
            let required_zeros = n - 1 - i;

            // Try to find a row with enough trailing zeros
            let mut found = false;
            for j in i..n {
                if trailing_zeros[j] >= required_zeros {
                    // Move this row up to position i
                    for k in (i..j).rev() {
                        trailing_zeros.swap(k, k + 1);
                        swaps += 1;
                    }
                    found = true;
                    break;
                }
            }

            if !found {
                return -1;
            }
        }

        swaps
    }
}

fn main() {}
