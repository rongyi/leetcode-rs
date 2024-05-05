#![allow(dead_code)]


struct Solution;

use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let s: Vec<_> = s.chars().collect();
        let mut char_cnt: HashMap<char, i32> = HashMap::new();
        for &c in s.iter() {
            *char_cnt.entry(c).or_insert(0) += 1;
        }
        let mut pq = BinaryHeap::new();
        for (&k, &v) in char_cnt.iter() {
            if v > (s.len() + 1) as i32 / 2 {
                return "".to_string();
            }
            pq.push((v, k));
        }
        let mut ret = String::new();
        while pq.len() >= 2 {
            let (mut cnt1, c1) = pq.pop().unwrap();
            let (mut cnt2, c2) = pq.pop().unwrap();
            ret.push(c1);
            ret.push(c2);
            cnt1 -= 1;
            cnt2 -= 1;
            if cnt1 > 0 {
                pq.push((cnt1, c1));
            }
            if cnt2 > 0 {
                pq.push((cnt2, c2));
            }
        }
        if !pq.is_empty() {
            ret.push(pq.pop().unwrap().1);
        }

        ret
    }
}

fn main() {}
