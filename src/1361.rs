#![allow(dead_code)]
struct Solution;

use std::collections::VecDeque;


impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let n = n as usize;
        let mut indegree = vec![0; n];
        for i in 0..n {
            if left_child[i] != -1 {
                indegree[left_child[i] as usize] += 1;
                if indegree[left_child[i] as usize] > 1 {
                    return false;
                }
            }
            if right_child[i] != -1 {
                indegree[right_child[i] as usize] += 1;
                if indegree[right_child[i] as usize] > 1 {
                    return false;
                }
            }
        }

        let mut root = -1;
        for i in 0..n {
            if indegree[i] == 0 {
                if root != -1 {
                    return false;
                }
                root = i as i32;
            }
        }
        // no root found
        if root == -1 {
            return false;
        }
        // now we have root, and doing bfs to check cycle
        let mut q: VecDeque<i32> = VecDeque::new();
        q.push_back(root);
        let mut visited = vec![false; n];
        visited[root as usize] = true;
        while let Some(cur) = q.pop_front() {
            if left_child[cur as usize] != -1 {
                let next = left_child[cur as usize];
                if visited[next as usize] {
                    return false;
                }
                q.push_back(next);
                visited[next as usize] = true;
            }
            if right_child[cur as usize] != -1 {
                let next = right_child[cur as usize];
                if visited[next as usize] {
                    return false;
                }
                q.push_back(next);
                visited[next as usize] = true;
            }
        }

        visited.iter().all(|&x| x)
    }
}

fn main() {}
