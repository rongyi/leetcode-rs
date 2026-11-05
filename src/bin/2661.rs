struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut fill_row = vec![HashSet::new(); m];
        let mut fill_col = vec![HashSet::new(); n];
        let mut index_cache: HashMap<i32, (usize, usize)> = HashMap::new();
        for i in 0..m {
            for j in 0..n {
                index_cache.insert(mat[i][j], (i, j));
            }
        }

        for (i, val) in arr.iter().enumerate() {
            let (x, y) = *index_cache.get(val).unwrap();
            fill_row[x].insert(y);
            fill_col[y].insert(x);
            if fill_row[x].len() == n || fill_col[y].len() == m {
                return i as i32;
            }
        }

        unreachable!()
    }
}

fn main() {
    let input = vec![vec![1, 4], vec![2, 3]];
    let arr = vec![1, 3, 4, 2];
    Solution::first_complete_index(arr, input);
}
