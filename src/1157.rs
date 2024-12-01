#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

struct MajorityChecker {
    arr: Vec<i32>,
    pos: HashMap<i32, Vec<usize>>, // val's position
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let mut pos = HashMap::new();
        for (i, &num) in arr.iter().enumerate() {
            pos.entry(num).or_insert(Vec::new()).push(i);
        }
        Self { arr, pos }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let (l, r) = (left as usize, right as usize);
        for _ in 0..20 {
            let candicate = self.arr[rand::random::<usize>() % (r - l + 1) + l];
            if let Some(pos) = self.pos.get(&candicate) {
                let cnt = self.count_in_range(pos, l, r);
                if cnt >= threshold {
                    return candicate;
                }
            }
        }

        -1
    }

    fn count_in_range(&self, postions: &Vec<usize>, left: usize, right: usize) -> i32 {
        let l = postions.partition_point(|&x| x < left);
        let r = postions.partition_point(|&x| x <= right);
        (r - l) as i32
    }
}

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * let obj = MajorityChecker::new(arr);
 * let ret_1: i32 = obj.query(left, right, threshold);
 */

fn main() {}
