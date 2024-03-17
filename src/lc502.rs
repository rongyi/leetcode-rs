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
        let mut idx = vec![0; sz];
        for (i, val) in idx.iter_mut().enumerate() {
            *val = i;
        }

        idx.sort_by(|l, r| capital[*l].cmp(&capital[*r]));

        let mut i = 0;
        while k > 0 {
            while i < sz && w >= capital[idx[i]] {
                pq.push(profits[idx[i]]);
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

fn main() {
    let mut idx = [0; 10];
    for (i, val) in idx.iter_mut().enumerate() {
        *val = i;
    }
    println!("{:?} ", idx);
}
