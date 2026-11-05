struct Solution;

use core::f64;
use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        // 1. 得按比例最小的人来，什么意思： 意思就是你看那谁谁谁，能力那么强才拿那么点，你们
        //    都学着点。
        // 2. 比例最小的来，大家总共拿多少？ ==> 最小的那个比例 ratio * (k sum quality)
        // 性价比之王排在最前面，躺着挣钱的在最后面
        let sz = quality.len();

        let mut workers = Vec::new();
        for i in 0..sz {
            let ratio: f64 = wage[i] as f64 / quality[i] as f64;
            workers.push((ratio, quality[i]));
        }
        workers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut pq = BinaryHeap::new();

        let mut ret: f64 = f64::MAX;
        let mut qsum = 0;

        for &(ratio, qlt) in workers.iter() {
            pq.push(qlt);
            qsum += qlt;

            if pq.len() > k as usize {
                qsum -= pq.pop().unwrap();
            }

            if pq.len() == k as usize {
                ret = ret.min(qsum as f64 * ratio);
            }
        }

        ret
    }
}

fn main() {}
