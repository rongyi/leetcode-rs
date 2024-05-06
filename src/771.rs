#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut ret = 0;
        for c in jewels.chars() {
            let mut cur = stones.clone();
            cur.retain(|c2| c2 == c);
            ret += cur.len() as i32;
        }

        ret
    }
}

fn main() {}
