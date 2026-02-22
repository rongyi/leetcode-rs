struct Solution;

impl Solution {
    pub fn min_edge_reversals(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let sz = n as usize;
        let mut graph = vec![vec![]; sz];
        for e in edges.iter() {
            let (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push((v, 0));
            // reverse weight is 1
            graph[v].push((u, 1));
        }
        let mut ret = vec![0; sz];

        fn dfs1(u: usize, p: usize, graph: &Vec<Vec<(usize, i32)>>) -> i32 {
            let mut cost = 0;
            for &(v, weight) in graph[u].iter() {
                if v != p {
                    cost += weight + dfs1(v, u, graph);
                }
            }

            cost
        }
        ret[0] = dfs1(0, 0, &graph);

        fn dfs2(u: usize, p: usize, graph: &Vec<Vec<(usize, i32)>>, ret: &mut Vec<i32>) {
            for &(v, weight) in graph[u].iter() {
                if v != p {
                    // u -> v has weight: weight, now v as root, so minus this weight and then add v -> u which is (1 - weight)
                    ret[v] = ret[u] - weight + (1 - weight);
                    dfs2(v, u, graph, ret);
                }
            }
        }
        dfs2(0, 0, &graph, &mut ret);

        ret
    }
}

fn main() {}
