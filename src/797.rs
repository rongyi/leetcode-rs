#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();

        // why no visted? because this is a DAG
        Self::dfs(&graph, 0, &mut Vec::new(), &mut ret);

        ret
    }

    fn dfs(graph: &Vec<Vec<i32>>, cur_node: usize, path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        path.push(cur_node as i32);

        if cur_node == graph.len() - 1 {
            ret.push(path.clone());
        }
        for &next in graph[cur_node].iter() {
            Self::dfs(graph, next as usize, path, ret);
        }

        path.pop();
    }
}

fn main() {
    let input = vec![1, 1, 1];
}
