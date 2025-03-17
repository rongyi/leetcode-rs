#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n + 1];

        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut result = vec![0; n - 1];

        // Iterate through all possible subsets of cities (except empty set)
        for mask in 1..(1 << n) {
            // Skip singleton sets
            if mask & (mask - 1) == 0 {
                continue;
            }

            // Check if the subset forms a connected subgraph
            let mut cities = vec![];
            for i in 1..=n {
                if (mask & (1 << (i - 1))) != 0 {
                    cities.push(i);
                }
            }

            if !Self::is_connected(&graph, &cities, mask) {
                continue;
            }

            // Calculate the diameter of the subgraph
            let diameter = Self::calculate_diameter(&graph, &cities, mask);
            if diameter > 0 {
                result[diameter - 1] += 1;
            }
        }

        result
    }

    // Check if the selected cities form a connected subgraph
    fn is_connected(graph: &Vec<Vec<usize>>, cities: &Vec<usize>, mask: i32) -> bool {
        if cities.is_empty() {
            return true;
        }

        let start = cities[0];
        let mut visited = vec![false; graph.len()];
        let mut count = 0;

        Self::dfs(graph, start, mask, &mut visited, &mut count);

        count == cities.len()
    }

    fn dfs(
        graph: &Vec<Vec<usize>>,
        node: usize,
        mask: i32,
        visited: &mut Vec<bool>,
        count: &mut usize,
    ) {
        visited[node] = true;
        *count += 1;

        for &neighbor in &graph[node] {
            if !visited[neighbor] && (mask & (1 << (neighbor - 1))) != 0 {
                Self::dfs(graph, neighbor, mask, visited, count);
            }
        }
    }

    // Calculate the diameter of the subgraph
    fn calculate_diameter(graph: &Vec<Vec<usize>>, cities: &Vec<usize>, mask: i32) -> usize {
        let mut diameter = 0;

        // For each city, find the farthest city from it
        for &start in cities {
            let mut distances = vec![usize::MAX; graph.len()];
            Self::bfs(graph, start, mask, &mut distances);

            let max_dist = cities
                .iter()
                .map(|&city| distances[city])
                .max()
                .unwrap_or(0);

            diameter = diameter.max(max_dist);
        }

        diameter
    }

    fn bfs(graph: &Vec<Vec<usize>>, start: usize, mask: i32, distances: &mut Vec<usize>) {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        queue.push_back(start);
        distances[start] = 0;

        while let Some(node) = queue.pop_front() {
            for &neighbor in &graph[node] {
                if (mask & (1 << (neighbor - 1))) != 0 && distances[neighbor] == usize::MAX {
                    distances[neighbor] = distances[node] + 1;
                    queue.push_back(neighbor);
                }
            }
        }
    }
}

fn main() {}
