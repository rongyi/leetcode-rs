#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let mut idx1 = Vec::new();
        let mut idx2 = Vec::new();
        let sz = img1.len();
        for i in 0..sz * sz {
            if img1[i / sz][i % sz] == 1 {
                idx1.push(i / sz * 100 + i % sz);
            }
            if img2[i / sz][i % sz] == 1 {
                idx2.push(i / sz * 100 + i % sz);
            }
        }
        let mut cnt = HashMap::new();
        for &a in idx1.iter() {
            for &b in idx2.iter() {
                *cnt.entry(a as i32 - b as i32).or_insert(0) += 1;
            }
        }
        let mut ret = 0;
        for (_k, &v) in cnt.iter() {
            ret = ret.max(v);
        }

        ret
    }
}

fn main() {}
