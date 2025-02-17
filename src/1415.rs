#![allow(dead_code)]

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut ret: Vec<Vec<char>> = Vec::new();
        let mut q: VecDeque<Vec<char>> = VecDeque::new();
        q.push_back(vec!['a']);
        q.push_back(vec!['b']);
        q.push_back(vec!['c']);

        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            if cur.len() == n {
                ret.push(cur.clone());
            }
            let mut s1: Vec<char> = cur.clone();
            let mut s2: Vec<char> = cur.clone();
            match *cur.last().unwrap() {
                'a' => {
                    s1.push('b');
                    s2.push('c');
                }
                'b' => {
                    s1.push('a');
                    s2.push('c');
                }
                'c' => {
                    s1.push('a');
                    s2.push('b');
                }
                _ => {
                    unreachable!("fail")
                }
            }

            if s1.len() <= n {
                q.push_back(s1);
            }

            if s2.len() <= n {
                q.push_back(s2);
            }
        }
        if k as usize >= ret.len() + 1 {
            return "".to_string();
        }
        ret[k as usize - 1].iter().collect()
    }
}
fn main() {}
