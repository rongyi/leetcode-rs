#![allow(dead_code)]

struct Solution;

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
impl Solution {
    pub fn watched_videos_by_friends(
        watched_videos: Vec<Vec<String>>,
        friends: Vec<Vec<i32>>,
        id: i32,
        mut level: i32,
    ) -> Vec<String> {
        let mut q = VecDeque::from_iter([id]);
        let mut visited: HashSet<i32> = HashSet::from_iter([id]);

        while level > 0 {
            let mut q2 = VecDeque::new();

            while let Some(cur) = q.pop_front() {
                for &next in friends[cur as usize].iter() {
                    if visited.insert(next) {
                        q2.push_back(next);
                    }
                }
            }

            level -= 1;
            q = q2;
        }
        let mut freq: HashMap<String, i32> = HashMap::new();
        for &id in q.iter() {
            for mov in watched_videos[id as usize].iter() {
                *freq.entry(mov.clone()).or_insert(0) += 1;
            }
        }
        let sorted: BTreeSet<(i32, String)> = freq.iter().map(|(k, &v)| (v, k.clone())).collect();

        sorted.into_iter().map(|(_, s)| s).collect()
    }
}

fn main() {}
