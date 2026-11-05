struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let sz = n as usize;
        // let mut out: Vec<Vec<String>> = vec![];
        let mut out: i32 = 0;
        let mut cur = vec![vec!['.'; sz]; sz];
        let mut rows: Vec<bool> = vec![false; sz];
        let mut cols: Vec<bool> = vec![false; sz];
        let mut diag1 = HashMap::new();
        let mut diag2 = HashMap::new();

        Self::dfs(
            &mut cur, 0, sz, &mut rows, &mut cols, &mut diag1, &mut diag2, &mut out,
        );

        out
    }
    fn dfs(
        cur: &mut Vec<Vec<char>>,
        x: usize,
        sz: usize,
        rows: &mut Vec<bool>,
        cols: &mut Vec<bool>,
        diag1: &mut HashMap<i32, bool>,
        diag2: &mut HashMap<i32, bool>,
        // out: &mut Vec<Vec<String>>,
        out: &mut i32,
    ) {
        if x == sz {
            // ok, a valid
            // let cur_s: Vec<String> = cur.iter().map(|vc| vc.iter().copied().collect()).collect();
            *out += 1;
            return;
        }

        for j in 0..sz {
            let k1 = x as i32 + j as i32;
            let k2 = x as i32 - j as i32;
            if cols[j] || *diag1.get(&k1).unwrap_or(&false) || *diag2.get(&k2).unwrap_or(&false) {
                continue;
            }

            cols[j] = true;
            diag1.insert(k1, true);
            diag2.insert(k2, true);
            cur[x][j] = 'Q';

            Self::dfs(cur, x + 1, sz, rows, cols, diag1, diag2, out);

            cur[x][j] = '.';
            cols[j] = false;
            diag1.remove(&k1);
            diag2.remove(&k2);
        }
    }
}

fn main() {}
