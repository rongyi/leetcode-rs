#![allow(dead_code)]

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let m = m as usize;
        // reformat group, make orhan group a uniq group number
        let group: Vec<usize> = group
            .into_iter()
            .enumerate()
            .map(|(i, g)| if g == -1 { i + m } else { g as usize })
            .collect();

        let mut group_graph: Vec<Vec<usize>> = vec![vec![]; m + n];
        let mut group_in_degress = vec![0; m + n];

        let mut item_graph: Vec<Vec<usize>> = vec![vec![]; n];
        let mut item_in_degress = vec![0; n];

        // build graph: deps should before cur_item
        for (cur_item, deps) in before_items.iter().enumerate() {
            for &prev_item in deps.iter() {
                let prev_item = prev_item as usize;
                // same group?
                if group[cur_item] == group[prev_item] {
                    item_graph[prev_item].push(cur_item);
                    item_in_degress[cur_item] += 1;
                } else {
                    group_graph[group[prev_item]].push(group[cur_item]);
                    group_in_degress[group[cur_item]] += 1;
                }
            }
        }
        let group_order = Self::topological_sort(&group_graph, &mut group_in_degress);
        if group_order.is_empty() {
            return Vec::new();
        }
        let item_order = Self::topological_sort(&item_graph, &mut item_in_degress);
        if item_order.is_empty() {
            return vec![];
        }

        let mut group_items: Vec<Vec<i32>> = vec![vec![]; m + n];
        for &item in &item_order {
            group_items[group[item]].push(item as i32);
        }
        let mut ret = Vec::new();
        for &group_id in group_order.iter() {
            ret.extend(&group_items[group_id]);
        }

        ret
    }

    fn topological_sort(graph: &Vec<Vec<usize>>, indegree: &mut Vec<i32>) -> Vec<usize> {
        let sz = graph.len();
        let mut ret = Vec::new();

        let mut q = VecDeque::new();
        // start from 0 in degree
        for i in 0..sz {
            if indegree[i] == 0 {
                q.push_back(i);
            }
        }

        while let Some(node) = q.pop_front() {
            ret.push(node);
            for &next in graph[node].iter() {
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
