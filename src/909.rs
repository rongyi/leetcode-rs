
struct Solution;

use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let sz = board.len();
        let n = (sz * sz) as i32;
        let mut step: HashMap<i32, i32> = HashMap::new();
        step.insert(1, 0);
        let mut q: VecDeque<i32> = VecDeque::new();
        q.push_back(1);
        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            if cur == n {
                return step[&cur];
            }
            for i in 1..=6 {
                let mut nxt = cur + i;
                if nxt > n {
                    break;
                }
                let (nx, ny) = Self::coord(nxt, sz as i32);
                if board[nx][ny] != -1 {
                    nxt = board[nx][ny];
                }
                if !step.contains_key(&nxt) {
                    step.insert(nxt, step[&cur] + 1);
                    q.push_back(nxt);
                }
            }
        }
        -1
    }

    fn coord(nxt: i32, n: i32) -> (usize, usize) {
        let mut x = (nxt - 1) / n;
        let mut y = (nxt - 1) % n;
        if x % 2 == 1 {
            y = n - y - 1;
        }
        x = n - 1 - x;
        (x as usize, y as usize)
    }
}

fn main() {}
