#![allow(dead_code)]

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn minimum_moves(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut steps = 0;
        // (i, j, is_vertical)
        let mut q1 = VecDeque::new();
        let mut q2 = VecDeque::new();
        // 一开始是横着的
        q1.push_back((0, 0, false));
        while !q1.is_empty() {
            while !q1.is_empty() {
                let cur = q1.pop_front().unwrap();
                // The snake wants to reach the lower right corner at (n-1, n-2) and (n-1, n-1)c
                if cur.0 == grid.len() - 1 && cur.1 == grid[0].len() - 2 {
                    return steps;
                }
                // 访问的标记位， 2表示竖着来过，4表示横着来过
                if grid[cur.0][cur.1] & if cur.2 { 2 } else { 4 } == 0 {
                    grid[cur.0][cur.1] |= if cur.2 { 2 } else { 4 };
                    if Self::can_go_down(&grid, cur.0, cur.1, cur.2) {
                        q2.push_back((cur.0 + 1, cur.1, cur.2));
                    }
                    if Self::can_go_right(&grid, cur.0, cur.1, cur.2) {
                        q2.push_back((cur.0, cur.1 + 1, cur.2));
                    }
                    if Self::can_rotate(&grid, cur.0, cur.1) {
                        q2.push_back((cur.0, cur.1, if cur.2 { false } else { true }))
                    }
                }
            }

            steps += 1;
            q1 = q2.clone();
            q2.clear();
        }

        return -1;
    }

    fn can_rotate(g: &Vec<Vec<i32>>, i: usize, j: usize) -> bool {
        i < g.len() - 1
            && j < g[0].len() - 1
            && (g[i + 1][j] & 1 == 0)
            && (g[i][j + 1] & 1 == 0)
            && (g[i + 1][j + 1] & 1 == 0)
    }
    fn can_go_down(g: &Vec<Vec<i32>>, i: usize, j: usize, is_vertical: bool) -> bool {
        if is_vertical {
            return i < g.len() - 2 && (g[i + 2][j] & 1) == 0;
        }

        // 横着的，下面两个要为空，这一步是平移
        i < g.len() - 1 && (g[i + 1][j] & 1 == 0) && (g[i + 1][j + 1] & 1 == 0)
    }

    fn can_go_right(g: &Vec<Vec<i32>>, i: usize, j: usize, is_vertical: bool) -> bool {
        if !is_vertical {
            return j < g[0].len() - 2 && g[i][j + 2] & 1 == 0;
        }
        //本身是竖着的，右边两个应该为空, 这一步是平移
        return j < g[i].len() - 1 && (g[i][j + 1] & 1) == 0 && (g[i + 1][j + 1] & 1) == 0;
    }
}
fn main() {}
