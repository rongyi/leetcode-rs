#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        let mut count = vec![0; n as usize];
        let labels: Vec<char> = labels.chars().collect();
        Self::dfs(0, -1, &graph, &mut count, &labels);
        count
    }

    fn dfs(
        node: i32,
        parent: i32,
        graph: &Vec<Vec<i32>>,
        count: &mut Vec<i32>,
        labels: &Vec<char>,
    ) -> Vec<i32> {
        let cur_color = labels[node as usize] as usize - 'a' as usize;
        let mut label_count = vec![0; 26];
        label_count[cur_color] += 1;

        for &next_node in graph[node as usize].iter() {
            if next_node != parent {
                let child = Self::dfs(next_node, node, graph, count, labels);
                for i in 0..26 {
                    label_count[i] += child[i];
                }
            }
        }

        count[node as usize] = label_count[cur_color as usize];

        label_count
    }
}
fn main() {}
