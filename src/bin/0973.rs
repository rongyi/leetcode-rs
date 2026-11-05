struct Solution;

mod ai {
    struct Solution;

    impl Solution {
        pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
            let k = k as usize;

            // QuickSelect: Partitions points so that the top `k` closest
            // elements are placed in indices `0..k`.
            points.select_nth_unstable_by_key(k - 1, |p| p[0] * p[0] + p[1] * p[1]);

            // Truncate to keep only the first `k` elements
            points.truncate(k);
            points
        }
    }
}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut pq = BinaryHeap::new();
        let mut distance: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();

        for p in points.iter() {
            let cur_dist = p[0] * p[0] + p[1] * p[1];
            pq.push(Reverse(cur_dist));
            distance.entry(cur_dist).or_insert(vec![]).push(p.clone());
        }
        let mut cur_cnt = 0;
        let mut ret = Vec::new();
        while cur_cnt < k {
            if let Some(Reverse(cur_dist)) = pq.pop() {
                let cur_points = distance.get(&cur_dist).unwrap();
                for p in cur_points.iter() {
                    ret.push(p.clone());
                    cur_cnt += 1;
                    if cur_cnt >= k {
                        break;
                    }
                }
            }
        }
        ret
    }
}

fn main() {}
