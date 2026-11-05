#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let sz = num_courses as usize;
        let mut dp = vec![vec![false; sz]; sz];

        for preq in prerequisites.iter() {
            let (from, to) = (preq[0] as usize, preq[1] as usize);
            dp[from][to] = true;
        }

        // i is the inner node
        for i in 0..sz {
            for j in 0..sz {
                for k in 0..sz {
                    if dp[j][i] && dp[i][k] {
                        dp[j][k] = true;
                    }
                }
            }
        }

        queries
            .into_iter()
            .map(|q| dp[q[0] as usize][q[1] as usize])
            .collect()
    }

    pub fn check_if_prerequisite1(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        let sz = num_courses as usize;
        let mut cache: Vec<Vec<bool>> = vec![vec![false; sz]; sz];
        for pair in prerequisites.iter() {
            let (from, to) = (pair[0], pair[1]);
            graph.entry(from as usize).or_default().push(to as usize);
        }
        for i in 0..sz {
            let mut visited = vec![false; sz];
            Self::dfs(i, i, &graph, &mut cache, &mut visited);
        }
        let mut ret: Vec<bool> = vec![false; queries.len()];
        for i in 0..queries.len() {
            let (start, to) = (queries[i][0] as usize, queries[i][1] as usize);
            ret[i] = cache[start][to];
        }

        ret
    }
    fn dfs(
        start: usize,
        cur: usize,
        graph: &HashMap<usize, Vec<usize>>,
        cache: &mut Vec<Vec<bool>>,
        visited: &mut Vec<bool>,
    ) {
        if visited[cur] {
            return;
        }
        visited[cur] = true;
        if start != cur {
            cache[start][cur] = true;
        }

        if let Some(next) = graph.get(&cur) {
            for &nx in next.iter() {
                cache[start][nx] = true;
                Self::dfs(start, nx, graph, cache, visited);
            }
        }
    }
}

fn main() {}
