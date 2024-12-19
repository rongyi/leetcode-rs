#![allow(dead_code)]


struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        // n nodes
        let n = n as usize;
        // m groups
        let m = m as usize;
        // make groupless item a uniq new group id, 0..=m-1 is reserved for real group, only after
        let group: Vec<usize> = group
            .into_iter()
            .enumerate()
            .map(|(i, g)| if g == -1 { i + m } else { g as usize })
            .collect();
        let mut group_indegree = vec![0; m + n];
        let mut group_graph: Vec<Vec<usize>> = vec![vec![]; m + n];

        let mut item_indegree: Vec<i32> = vec![0; n];
        let mut item_graph: Vec<Vec<usize>> = vec![vec![]; n];

        // construct relationship
        for (cur_item, prev_items) in before_items.iter().enumerate() {
            for &prev_item in prev_items.iter() {
                let prev_item = prev_item as usize;
                // same group?
                if group[prev_item] == group[cur_item] {
                    item_graph[prev_item].push(cur_item);
                    item_indegree[cur_item] += 1;
                } else {
                    group_indegree[group[cur_item]] += 1;
                    group_graph[group[prev_item]].push(group[cur_item]);
                }
            }
        }
        let group_order = Self::topology_sort(&group_graph, &mut group_indegree);
        if group_order.is_empty() {
            return vec![];
        }
        let item_order = Self::topology_sort(&item_graph, &mut item_indegree);
        if item_order.is_empty() {
            return vec![];
        }
        // so now we ca group item in group and insert to a ret by group order
        let mut by_group: Vec<Vec<i32>> = vec![vec![]; m + n];
        for &item in item_order.iter() {
            by_group[group[item]].push(item as i32);
        }
        let mut ret = Vec::new();
        for &gid in group_order.iter() {
            ret.extend(&by_group[gid]);
        }

        ret
    }

    // start from indegree 0 and go
    fn topology_sort(graph: &Vec<Vec<usize>>, indegree: &mut Vec<i32>) -> Vec<usize> {
        let sz = graph.len();
        let mut q: VecDeque<usize> = VecDeque::new();

        // start from 0 indegree
        for i in 0..sz {
            if indegree[i] == 0 {
                q.push_back(i);
            }
        }

        let mut ret = Vec::new();
        while let Some(cur) = q.pop_front() {
            ret.push(cur);
            for &next in graph[cur].iter() {
                indegree[next] -= 1;
                if indegree[next] == 0 {
                    q.push_back(next);
                }
            }
        }

        if ret.len() == sz {
            ret
        } else {
            vec![]
        }
    }
}

fn main() {}
