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
        let n = n as usize;
        let mut degree = vec![0; n];
        let mut neibs: Vec<Vec<usize>> = vec![vec![]; n];

        for e in edges.iter() {
            let e0 = e[0];
            let e1 = e[1];
            neibs[e0 as usize].push(e1 as usize);
            neibs[e1 as usize].push(e0 as usize);
            degree[e0 as usize] += 1;
            degree[e1 as usize] += 1;
        }

        let mut node_left = n;
        while node_left > 2 {
            let mut outer_layer: Vec<usize> = Vec::new();
            for i in 0..n {
                if degree[i] == 1 {
                    outer_layer.push(i);
                    degree[i] = -1;
                }
            }
            // outer most node tobe deleted
            for &leaf in &outer_layer {
                // there inner conection minus a node
                for &next in &neibs[leaf] {
                    degree[next] -= 1;
                }
                node_left -= 1;
            }
        }

        let mut ret = vec![];
        for i in 0..n {
            if degree[i] >= 0 {
                ret.push(i as i32);
            }
        }
        ret
    }
}

fn main() {}
