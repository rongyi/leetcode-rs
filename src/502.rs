struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn find_maximized_capital(
        mut k: i32,
        mut w: i32,
        profits: Vec<i32>,
        capital: Vec<i32>,
    ) -> i32 {
        let mut pq = BinaryHeap::new();
        let sz = profits.len();

        let mut pc: Vec<(i32, i32)> = profits.into_iter().zip(capital.into_iter()).collect();
        pc.sort_by_key(|p| p.1);

        let mut i = 0;

        while k > 0 {
            while i < sz && w >= pc[i].1 {
                pq.push(pc[i].0);
                i += 1;
            }
            if pq.is_empty() {
                return w;
            }
            w += pq.pop().unwrap();

            k -= 1;
        }

        w
    }
}

fn main() {}
