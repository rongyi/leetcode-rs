#![allow(dead_code)]
struct Solution;

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
            // x个节点已知的distance之和和y
            // 现在x作为结合‘挂’在了一个新的节点 y上，如下图
            //    y
            //   /
            //  x
            // 这样的化x已经数好的distance到y会全部多一跳， 总共多少？ 集合X的节点数
            // 公式就是以下
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
            // https://leetcode.com/problems/sum-of-distances-in-tree/discuss/161975/My-DFS-sulotion-two-passes
            // (1) we divide the tree into two parts(two dotted circles).
            // (2) the total number of k subtree is num[k], and pis farther of k, the total number of p subtree except k subtree is N - num[k].
            // (3) assume we have calculated node p, now we should calculate node k, each node in k subtree should decrease by 1, and each node in p subtree except k subtree should increase by 1.
            // (4) so we get ans[k] = ans[p] - num[k] * 1 + (N - num[k]) * 1, which is ans[k] = ans[p] + N - 2 * num[k].
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
