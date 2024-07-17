#![allow(dead_code)]
struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited: HashSet<i32> = HashSet::new();
        let mut q: VecDeque<i32> = VecDeque::new();
        q.push_back(0);
        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            visited.insert(cur);
            if visited.len() == rooms.len() {
                return true;
            }
            for &next in rooms[cur as usize].iter() {
                if !visited.contains(&next) {
                    q.push_back(next);
                }
            }
        }

        visited.len() == rooms.len()
    }
}

fn main() {}
