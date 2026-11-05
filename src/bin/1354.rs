#![allow(dead_code)]

struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let target: Vec<i64> = target.into_iter().map(|x| x as i64).collect();
        let mut s: i64 = target.iter().sum();

        let mut q: BinaryHeap<i64> = target.iter().copied().collect();

        while s > 1 && *q.peek().unwrap() > s / 2 {
            let cur = q.pop().unwrap();

            s -= cur;
            if s <= 1 {
                if s == 1 {
                    return true;
                } else {
                    return false;
                }
            }

            // 这样去理解就可以了
            // q.push(cur - s);
            // s += cur - s;
            // 遇到有个数特别大的情况，需要碾掉,因为余下的那些数被加到这个数上很多次
            // 这是我见过 % 用的最巧妙的地方了
            q.push(cur % s);
            s += cur % s;
        }

        s == target.len() as i64
    }
}
fn main() {}
