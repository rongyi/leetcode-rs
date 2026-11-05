#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for conns in connections.iter() {
            graph[conns[0] as usize].push(conns[1]);
            graph[conns[1] as usize].push(conns[0]);
        }
        let mut ret = vec![];
        // not just identify visited or not, but also with when to visited this node
        let mut visits = vec![-1; n];
        // try our best to lower the visit time of one node
        let mut low = vec![i32::MAX; n];
        // assume one node consume 1 time unit for visiting
        let mut time = 0;

        // Yes, exactly! You've grasped a key insight of the algorithm. When DFS returns from a child node v, we have complete information about:

        // All nodes in v's subtree have been visited
        // The low[v] value is finalized, containing the lowest discovery time reachable from v's subtree
        // This is the perfect moment to check if edge (u,v) is a bridge by comparing low[v] and disc[u]
        Self::dfs(0, -1, &mut time, &mut visits, &mut low, &mut ret, &graph);

        ret
    }

    fn dfs(
        u: usize,
        parent: i32,
        time: &mut i32,
        visited: &mut Vec<i32>,
        low: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
        graph: &Vec<Vec<i32>>,
    ) {
        visited[u] = *time;
        low[u] = low[u].min(*time);
        *time += 1;

        for &v in graph[u].iter() {
            if visited[v as usize] == -1 {
                Self::dfs(v as usize, u as i32, time, visited, low, ret, graph);
                low[u] = low[u].min(low[v as usize]);
                if low[v as usize] > visited[u] {
                    ret.push(vec![u as i32, v]);
                }
            } else if v != parent {
                low[u] = low[u].min(visited[v as usize]);
            }
        }
    }
}

fn main() {}
