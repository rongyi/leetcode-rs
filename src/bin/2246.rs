struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let s = s.as_bytes();
        let sz = parent.len();
        // only 1 edge
        let mut ret = 1;
        // record child number
        let mut child_count = vec![0; sz];
        for i in 1..sz {
            child_count[parent[i] as usize] += 1;
        }

        let mut q = VecDeque::new();

        for i in 0..sz {
            if child_count[i] == 0 {
                q.push_back(i);
            }
        }

        let mut top1 = vec![1; sz];
        let mut top2 = vec![1; sz];
        while !q.is_empty() && *q.front().unwrap() != 0 {
            let cur = q.pop_front().unwrap();
            let cur_parent = parent[cur] as usize;

            let cur_len = 1 + if s[cur] != s[cur_parent] {
                top1[cur]
            } else {
                0
            };
            if top1[cur_parent] <= cur_len {
                top2[cur_parent] = top1[cur_parent];
                top1[cur_parent] = cur_len;
            } else {
                top2[cur_parent] = top2[cur_parent].max(cur_len);
            }
            child_count[cur_parent] -= 1;
            if child_count[cur_parent] == 0 {
                q.push_back(cur_parent);
                ret = ret.max(top1[cur_parent] + top2[cur_parent] - 1);
            }
        }

        ret
    }
}

fn main() {}
