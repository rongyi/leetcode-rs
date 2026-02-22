struct Solution;

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let sz = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; sz];
        for e in edges.iter() {
            let (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut ret = 0;

        fn dfs(
            u: usize,
            p: usize,
            graph: &Vec<Vec<usize>>,
            values: &Vec<i32>,
            k: i64,
            count: &mut i32,
        ) -> i64 {
            let mut sum: i64 = values[u] as i64;
            for &v in graph[u].iter() {
                if v != p {
                    sum += dfs(v, u, graph, values, k, count);
                }
            }
            if sum % k == 0 {
                *count += 1;
                return 0;
            }
            sum % k
        }
        dfs(0, 0, &graph, &values, k as i64, &mut ret);

        ret
    }
}

fn main() {}
