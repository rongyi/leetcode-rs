#![allow(dead_code)]

use std::collections::HashMap;

use rand::Rng;

struct Solution {
    m: i32,
    black_map: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let mut ret = Self {
            m: n - blacklist.len() as i32,
            black_map: HashMap::new(),
        };
        for &b in blacklist.iter() {
            ret.black_map.insert(b, -1);
        }

        let mut cur = n - 1;
        println!("m: {}", ret.m);
        // remap blacklist value in namespace [0..m) -> [0..n)
        // https://leetcode.com/problems/random-pick-with-blacklist/solutions/144624/java-o-b-o-1-hashmap/
        // there's a pic there, just see the picture, you will know
        for &b in blacklist.iter() {
            if b < ret.m {
                while ret.black_map.contains_key(&cur) {
                    cur -= 1;
                }
                // println!("here insert: k: {}, v: {}", b, cur);
                ret.black_map.insert(b, cur);
                cur -= 1;
            }
        }

        ret
    }

    fn pick(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let before_map = rng.gen_range(0..self.m);
        if self.black_map.contains_key(&before_map) {
            return self.black_map[&before_map];
        }
        before_map
    }
}

fn main() {
    let input = vec![0, 1];
    let s = Solution::new(4, input);
    println!("{}", s.pick());
}
