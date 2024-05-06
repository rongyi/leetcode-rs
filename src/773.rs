#![allow(dead_code)]

struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let target = "123450".to_string();
        let dirs = vec![
            vec![1, 3],
            vec![0, 2, 4],
            vec![1, 5],
            vec![0, 4],
            vec![1, 3, 5],
            vec![2, 4],
        ];
        let m = board.len();
        let n = board[0].len();
        let mut start = String::new();
        for i in 0..m {
            for j in 0..n {
                start.push_str(&board[i][j].to_string());
            }
        }
        let mut q = VecDeque::new();
        q.push_back(start);

        let mut visited: HashSet<String> = HashSet::new();
        let mut ret = 0;
        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let cur = q.pop_front().unwrap();
                // yeah
                if cur == target {
                    return ret;
                }
                let zero_idx = cur.find('0').unwrap();
                for &next in dirs[zero_idx].iter() {
                    let mut cp: Vec<char> = cur.chars().collect();
                    cp.swap(zero_idx, next);
                    let cps: String = cp.into_iter().collect();
                    if !visited.contains(&cps) {
                        visited.insert(cps.clone());
                        q.push_back(cps);
                    }
                }
            }

            ret += 1;
        }

        -1
    }
}

fn main() {
    let input = vec![vec![1, 2, 3], vec![4, 0, 5]];
    Solution::sliding_puzzle(input);
}
