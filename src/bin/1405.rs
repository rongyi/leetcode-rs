#![allow(dead_code)]


struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap = BinaryHeap::new();
        if a > 0 {
            heap.push((a, 'a'));
        }
        if b > 0 {
            heap.push((b, 'b'));
        }
        if c > 0 {
            heap.push((c, 'c'));
        }
        let mut ret = String::new();
        let mut last_two = (' ', ' ');
        while let Some((count, c)) = heap.pop() {
            if c == last_two.0 && c == last_two.1 {
                if let Some((next_cnt, next_c)) = heap.pop() {
                    ret.push(next_c);
                    last_two = (last_two.1, next_c);
                    if next_cnt > 1 {
                        heap.push((next_cnt - 1, next_c));
                    }
                    // current count is still valid, push to heap
                    heap.push((count, c));
                } else {
                    // no valid choice
                    break;
                }
            } else {
                ret.push(c);
                last_two = (last_two.1, c);
                if count > 1 {
                    heap.push((count - 1, c));
                }
            }
        }

        ret
    }
}

fn main() {}
