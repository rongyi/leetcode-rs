struct Solution;

impl Solution {
    // take for max, so we left for min
    pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
        let sz = values.len();
        let mut graph: Vec<Vec<usize>> = vec![vec![]; sz];

        for e in edges.iter() {
            let (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }
        let total_sum: i64 = values.iter().map(|&x| x as i64).sum();

        fn dfs(u: usize, p: usize, graph: &Vec<Vec<usize>>, values: &Vec<i32>) -> i64 {
            // check u is leaf?
            if u != 0 && graph[u].len() == 1 {
                return values[u] as i64;
            }
            let mut sub_sum = 0;
            for &v in graph[u].iter() {
                if v != p {
                    sub_sum += dfs(v, u, graph, values);
                }
            }
            (values[u] as i64).min(sub_sum)
        }

        let left_in_tree = dfs(0, 0, &graph, &values);

        total_sum - left_in_tree
    }
}

fn main() {}
