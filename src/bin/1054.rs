struct Solution;

use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let sz = barcodes.len();
        let mut ret = vec![0; sz];
        let mut freq_map = HashMap::new();
        for &code in barcodes.iter() {
            *freq_map.entry(code).or_insert(0) += 1;
        }
        let mut pq = BinaryHeap::new();
        for (&num, &cnt) in freq_map.iter() {
            pq.push((cnt, num));
        }
        let mut i = 0;
        while pq.len() >= 2 {
            let (cnt1, val1) = pq.pop().unwrap();
            let (cnt2, val2) = pq.pop().unwrap();
            ret[i] = val1;
            ret[i + 1] = val2;
            i += 2;
            if cnt1 > 1 {
                pq.push((cnt1 - 1, val1));
            }
            if cnt2 > 1 {
                pq.push((cnt2 - 1, val2));
            }
        }
        if let Some((_cnt, val)) = pq.pop() {
            ret[i] = val;
        }

        ret
    }
}

fn main() {}
