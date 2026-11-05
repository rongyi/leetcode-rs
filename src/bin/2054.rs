pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort();
        let mut max_sum = 0;
        let mut max_sofar = 0;
        // make smaller go out first
        let mut pq: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

        for e in events.iter() {
            let (start, end, value) = (e[0], e[1], e[2]);
            while !pq.is_empty() {
                let p = pq.peek().unwrap();
                if p.0 .0 < start {
                    max_sofar = max_sofar.max(p.0 .1);
                    pq.pop();
                } else {
                    break;
                }
            }
            max_sum = max_sum.max(max_sofar + value);

            pq.push(Reverse((end, value)));
        }

        max_sum
    }
}

fn main() {}
