#![allow(dead_code)]

struct Solution;

use std::ops::{BitAnd, BitOr};
#[derive(Debug, Clone)]
struct BitSet {
    vals: Vec<i32>,
}

impl BitSet {
    fn new() -> Self {
        BitSet { vals: vec![0; 26] }
    }

    fn set(&mut self, c: char) {
        let idx = (c as u8 - 'a' as u8) as usize;
        self.vals[idx] = 1;
    }

    fn count(&self) -> usize {
        self.vals.iter().filter(|&&val| val == 1).count()
    }

    fn any(&self) -> bool {
        self.vals.iter().any(|&val| val == 1)
    }
}

impl BitOr for &BitSet {
    type Output = BitSet;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut out = BitSet::new();
        for i in 0..26 {
            out.vals[i] = self.vals[i] | rhs.vals[i];
        }

        out
    }
}

impl BitAnd for &BitSet {
    type Output = BitSet;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut out = BitSet::new();
        for i in 0..26 {
            out.vals[i] = self.vals[i] & rhs.vals[i];
        }

        out
    }
}

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut dp: Vec<BitSet> = vec![];
        dp.push(BitSet::new());
        let mut ret = 0;
        for s in arr.iter() {
            let mut cur = BitSet::new();
            for c in s.chars() {
                cur.set(c);
            }
            // this string itself has duplicate, we can't use this string
            let n = cur.count();
            if n < s.len() {
                continue;
            }
            for i in (0..dp.len()).rev() {
                let prev = &dp[i].clone();
                if (prev & &cur).any() {
                    continue;
                }
                dp.push(prev | &cur);
                ret = ret.max(prev.count() + n);
            }
        }

        ret as i32
    }
}

fn main() {}
