#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let rows = box_grid.len();
        let cols = box_grid[0].len();
        let mut result = vec![vec!['.'; rows]; cols];

        // Rotate 90° clockwise and process each original row
        for i in 0..rows {
            let mut j = 0;
            while j < cols {
                // Find the position where stones should settle after rotation
                if box_grid[i][j] != '.' {
                    // If it's a stone or obstacle, place it in the rotated grid
                    let new_col = rows - 1 - i;
                    if box_grid[i][j] == '#' {
                        // For stones, find where they would fall
                        let mut k = j;
                        while k + 1 < cols && box_grid[i][k + 1] == '.' {
                            k += 1;
                        }
                        // Place the stone at the appropriate position
                        result[k][new_col] = '#';
                        // If stone didn't move, mark its original position as empty
                        if k != j {
                            result[j][new_col] = '.';
                        }
                    } else if box_grid[i][j] == '*' {
                        // Place obstacle
                        result[j][new_col] = '*';
                    }
                }
                j += 1;
            }
        }

        // Process gravity after rotation
        for col in 0..rows {
            let mut empty_pos = cols - 1;
            for row in (0..cols).rev() {
                if result[row][col] == '*' {
                    empty_pos = row - 1;
                } else if result[row][col] == '#' {
                    if row != empty_pos {
                        result[empty_pos][col] = '#';
                        result[row][col] = '.';
                    }
                    empty_pos -= 1;
                }
            }
        }

        result
    }
}

fn main() {}
