#![allow(dead_code)]

struct Solution;

use std::cmp::Ordering;

#[derive(PartialEq)]
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let mut graph = vec![vec![]; n as usize];
        for (i, edge) in edges.iter().enumerate() {
            graph[edge[0] as usize].push((edge[1], succ_prob[i]));
            graph[edge[1] as usize].push((edge[0], succ_prob[i]));
        }
        let mut dist = vec![0.0; n as usize];
        dist[start_node as usize] = 1.0;
        let mut pq = std::collections::BinaryHeap::new();
        pq.push((OrderedFloat(1.0), start_node));
        while let Some((prob, node)) = pq.pop() {
            if node == end_node {
                return prob.0;
            }
            for &(next, weight) in &graph[node as usize] {
                let new_prob = prob.0 * weight;
                if new_prob > dist[next as usize] {
                    dist[next as usize] = new_prob;
                    pq.push((OrderedFloat(new_prob), next));
                }
            }
        }
        0.0
    }
}

fn main() {}
