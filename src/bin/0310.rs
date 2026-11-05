struct Solution;

impl Solution {
    // The basic idea is "keep deleting leaves layer-by-layer, until reach the
    // root." Specifically, first find all the leaves, then remove them. After
    // removing, some nodes will become new leaves. So we can continue remove
    // them. Eventually, there is only 1 or 2 nodes left. If there is only one
    // node left, it is the root. If there are 2 nodes, either of them could be a
    // possible root. Time Complexity: Since each node will be removed at most
    // once, the complexity is O(n).
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let sz = n as usize;
        let mut degree = vec![0; sz];
        let mut graph: Vec<Vec<usize>> = vec![vec![]; sz];
        for e in edges.iter() {
            let (from, to) = (e[0] as usize, e[1] as usize);
            graph[from].push(to);
            graph[to].push(from);
            degree[from] += 1;
            degree[to] += 1;
        }
        let mut nodes_left = sz;
        while nodes_left > 2 {
            // find degree == 1
            let mut cur_layer = vec![];

            for i in 0..sz {
                if degree[i] == 1 {
                    cur_layer.push(i);
                    // mark as negative, not decrement
                    degree[i] = -1;
                }
            }
            // peeling those nodes
            for &leaf in cur_layer.iter() {
                for &neib in graph[leaf].iter() {
                    // delete this connection
                    degree[neib] -= 1;
                }
                nodes_left -= 1;
            }
        }

        degree
            .into_iter()
            .enumerate()
            .filter(|x| x.1 >= 0)
            .map(|x| x.0 as i32)
            .collect()
    }
}

fn main() {}
