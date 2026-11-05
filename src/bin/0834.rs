struct Solution;

mod ai {
    struct Solution;

    impl Solution {
        pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
            let n = n as usize;
            // Build adjacency list
            let mut graph = vec![Vec::new(); n];
            for edge in edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                graph[u].push(v);
                graph[v].push(u);
            }

            // Arrays to store subtree sizes and answer
            let mut size = vec![0; n];
            let mut ans = vec![0; n];

            // First DFS: compute subtree sizes and sum of distances from root (node 0)
            Self::dfs1(0, n, &graph, &mut size, &mut ans);

            // Second DFS: compute answers for all nodes using rerooting technique
            Self::dfs2(0, n, &graph, &mut size, &mut ans);

            ans
        }

        // First DFS: post-order traversal
        // Returns subtree size and contributes to ans[0] (sum of distances from root)
        fn dfs1(
            u: usize,
            parent: usize,
            graph: &Vec<Vec<usize>>,
            size: &mut Vec<i32>,
            ans: &mut Vec<i32>,
        ) {
            size[u] = 1;
            for &v in &graph[u] {
                if v == parent {
                    continue;
                }
                Self::dfs1(v, u, graph, size, ans);
                size[u] += size[v];
                ans[0] += size[v]; // Each node in v's subtree contributes 1 to distance from root
            }
        }

        // Second DFS: pre-order traversal with rerooting
        fn dfs2(
            u: usize,
            parent: usize,
            graph: &Vec<Vec<usize>>,
            size: &Vec<i32>,
            ans: &mut Vec<i32>,
        ) {
            for &v in &graph[u] {
                if v == parent {
                    continue;
                }
                // Reroot from u to v
                // When moving root from u to v:
                // - Nodes in v's subtree get 1 closer (size[v] nodes)
                // - All other nodes get 1 farther (n - size[v] nodes)
                let n = ans.len() as i32;
                ans[v] = ans[u] - size[v] + (n - size[v]);
                Self::dfs2(v, u, graph, size, ans);
            }
        }
    }
}

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g: Vec<Vec<i32>> = vec![vec![]; n as usize];
        for e in edges.iter() {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let mut num_dis: Vec<(i32, i32)> = vec![(-1, -1); n as usize];

        Self::dfs(&g, &mut num_dis, 0, -1);
        let mut ret: Vec<i32> = vec![-1; n as usize];

        Self::dfs2(&g, &num_dis, &mut ret, 0, -1, n);

        ret
    }

    fn dfs(g: &Vec<Vec<i32>>, num_dis: &mut Vec<(i32, i32)>, cur: i32, pre: i32) {
        let mut node_count = 1;
        let mut total_distance = 0;

        for &next_node in g[cur as usize].iter() {
            if next_node == pre {
                continue;
            }
            Self::dfs(g, num_dis, next_node, cur);

            node_count += num_dis[next_node as usize].0;
            // it's subtotal path and from parent to this subtree(including itself), the single path
            // is going though size(subtree) times
            total_distance += num_dis[next_node as usize].1 + num_dis[next_node as usize].0;
        }

        num_dis[cur as usize] = (node_count, total_distance);
    }

    fn dfs2(
        g: &Vec<Vec<i32>>,
        num_dis: &Vec<(i32, i32)>,
        ret: &mut Vec<i32>,
        cur: i32,
        pre: i32,
        n: i32,
    ) {
        if pre == -1 {
            ret[cur as usize] = num_dis[cur as usize].1;
        } else {
            // Reroot from u to v
            // When moving root from u to v:
            // - Nodes in v's subtree get 1 closer (size[v] nodes)
            // - All other nodes get 1 farther (n - size[v] nodes)
            ret[cur as usize] = ret[pre as usize] + n - 2 * num_dis[cur as usize].0;
        }

        for &next_node in g[cur as usize].iter() {
            if next_node != pre {
                Self::dfs2(g, num_dis, ret, next_node, cur, n);
            }
        }
    }
}

fn main() {}
