struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let sz = n as usize;
        let mut out: Vec<Vec<String>> = vec![];
        let mut cur = vec![vec!['.'; sz]; sz];
        let mut rows: Vec<bool> = vec![false; sz];
        let mut cols: Vec<bool> = vec![false; sz];
        let mut diag1 = HashMap::new();
        let mut diag2 = HashMap::new();

        Self::dfs(
            &mut cur, 0, 0, sz, &mut rows, &mut cols, &mut diag1, &mut diag2, n, &mut out,
        );

        out
    }
    fn dfs(
        cur: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
        sz: usize,
        rows: &mut Vec<bool>,
        cols: &mut Vec<bool>,
        diag1: &mut HashMap<i32, bool>,
        diag2: &mut HashMap<i32, bool>,
        queue_left: i32,
        out: &mut Vec<Vec<String>>,
    ) {
        if x == sz {
            // ok, a valid
            if queue_left == 0 {
                let cur_s: Vec<String> =
                    cur.iter().map(|vc| vc.iter().copied().collect()).collect();
                out.push(cur_s);
            }
            return;
        }
        if y == sz {
            return Self::dfs(cur, x + 1, 0, sz, rows, cols, diag1, diag2, queue_left, out);
        }
        // 1. first I can put '.' in this pos, right?
        // because init with '.' so no put action
        Self::dfs(cur, x, y + 1, sz, rows, cols, diag1, diag2, queue_left, out);
        // 2. we put 'Q' in this position but only with valid case
        if queue_left > 0
            && !rows[x]
            && !cols[y]
            && !diag1.get(&((x + y) as i32)).unwrap_or(&false)
            && !diag2.get(&(x as i32 - y as i32)).unwrap_or(&false)
        {
            cur[x][y] = 'Q';
            rows[x] = true;
            cols[y] = true;
            diag1.insert(x as i32 + y as i32, true);
            diag2.insert(x as i32 - y as i32, true);

            Self::dfs(
                cur,
                x,
                y + 1,
                sz,
                rows,
                cols,
                diag1,
                diag2,
                queue_left - 1,
                out,
            );

            cur[x][y] = '.';
            rows[x] = false;
            cols[y] = false;
            diag1.remove(&(x as i32 + y as i32));
            diag2.remove(&(x as i32 - y as i32));
        }
    }
}

fn main() {}
