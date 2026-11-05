struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let q_sz = queries.len();
        let mut ret = vec![-1; q_sz];
        let mut defered: Vec<Vec<(i32, usize)>> = vec![vec![]; heights.len()];
        for (qidx, q) in queries.iter().enumerate() {
            let (i, j) = (q[0].min(q[1]) as usize, q[0].max(q[1]) as usize);
            if i == j || heights[i] < heights[j] {
                ret[qidx] = j as i32;
            } else {
                defered[j].push((heights[i], qidx));
            }
        }
        let mut min_heap = BinaryHeap::new();

        for (i, &cur_height) in heights.iter().enumerate() {
            // mainly answer this quetion,
            // now I am at height with value H, does some query have smaller value before me?
            // let's find out in this min_heap, and update it's nearest right pos is at my postion.
            while let Some(Reverse((prev_height, qidx))) = min_heap.peek() {
                if cur_height > *prev_height {
                    ret[*qidx] = i as i32;
                    min_heap.pop();
                } else {
                    break;
                }
            }

            for &d in defered[i].iter() {
                min_heap.push(Reverse(d));
            }
        }

        ret
    }
}

fn main() {}
