struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
        let mut q: BinaryHeap<i32> = BinaryHeap::new();
        for &m in amount.iter() {
            if m > 0 {
                q.push(m);
            }
        }
        let mut acc = 0;
        while !q.is_empty() {
            if q.len() >= 2 {
                let v1 = q.pop().unwrap();
                let v2 = q.pop().unwrap();
                acc += 1;
                if v1 > 1 {
                    q.push(v1 - 1);
                }
                if v2 > 1 {
                    q.push(v2 - 1);
                }
            } else {
                acc += q.pop().unwrap();
            }
        }

        acc
    }
}
fn main() {}
