#![allow(dead_code)]

struct Solution;

use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let sz = arr.len();
        let mut visited = vec![false; sz];
        visited[0] = true;
        // cache index
        let mut index: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..sz {
            index.entry(arr[i]).or_default().push(i);
        }

        let mut q = VecDeque::new();
        q.push_back(0);
        let mut step = 0;

        while !q.is_empty() {
            let qsz = q.len();
            for _ in 0..qsz {
                let cur = q.pop_front().unwrap();
                if cur == sz - 1 {
                    return step;
                }
                let mut next = index.get(&arr[cur]).unwrap_or(&Vec::default()).clone();
                next.push(cur + 1);
                next.push(cur - 1);
                for j in next.into_iter() {
                    if j >= arr.len() || visited[j] {
                        continue;
                    }
                    visited[j] = true;
                    q.push_back(j);
                }
                // 所有相等的value只进入队列一次
                // 后面就只选相邻的节点了
                index.remove(&arr[cur]);
            }
            step += 1;
        }

        todo!()
    }
}

fn main() {}
