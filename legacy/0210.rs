
struct Solution;

use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let sz = num_courses as usize;
        let mut ingress = vec![0; sz];
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        let mut ret: Vec<i32> = Vec::new();

        let mut q: VecDeque<i32> = VecDeque::new();
        for pair in &prerequisites {
            ingress[pair[0] as usize] += 1;
            // [0, 1] : 1 -> 0
            graph.entry(pair[1]).or_default().push(pair[0]);
        }
        for (i, degree) in ingress.iter().enumerate() {
            if *degree == 0 {
                q.push_back(i as i32);
            }
        }
        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            ret.push(cur);
            if let Some(neibs) = graph.get(&cur) {
                for &neib in neibs {
                    ingress[neib as usize] -= 1;
                    if ingress[neib as usize] == 0 {
                        q.push_back(neib);
                    }
                }
            }
        }

        if ret.len() != sz {
            ret.clear();
        }
        ret
    }
}

fn main() {}
