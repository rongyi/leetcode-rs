
struct Solution;
use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        // arrive, leave, idx
        let mut with_idx: Vec<Vec<i32>> = times
            .into_iter()
            .enumerate()
            .map(|(idx, mut val)| {
                val.push(idx as i32);
                val
            })
            .collect();
        // by arriving time
        with_idx.sort_by_key(|val| val[0]);
        let mut occ: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        let mut ready_chains: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut min_chair = 0;

        for v in with_idx.into_iter() {
            let (arrive, leave, index) = (v[0], v[1], v[2]);
            let mut cur_chair = -1;

            while let Some(Reverse((end, chair))) = occ.pop() {
                if end <= arrive {
                    ready_chains.push(Reverse(chair));
                } else {
                    occ.push(Reverse((end, chair)));
                    break;
                }
            }

            if let Some(Reverse(chair)) = ready_chains.pop() {
                cur_chair = chair;
            } else {
                cur_chair = min_chair;
                min_chair += 1;
            }

            occ.push(Reverse((leave, cur_chair)));
            if index == target_friend {
                return cur_chair;
            }
        }

        -1
    }
}

fn main() {}
