#![allow(dead_code)]
struct Solution;

impl Solution {
    // 分析： 有两种情况：
    // 1. 一个环，所有node都只有一个parent
    // 2. 有个node有两个parent, 另外一个没有parent，可能有一个环也可能没有环
    // https://leetcode.com/problems/redundant-connection-ii/discuss/138593/C%2B%2B-very-straight-forward-solution-either-node-has-two-parents-no-parent
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parent = vec![0; edges.len() + 1];
        let mut cycle = vec![0, 0];
        let mut visited = vec![false; edges.len() + 1];

        let mut target = 0;
        let mut p1 = 0;
        let mut p2 = 0;

        for e in edges.iter() {
            let (e0, e1) = (e[0] as usize, e[1] as usize);
            if visited[e0] && visited[e1] {
                cycle = vec![e0 as i32, e1 as i32];
            } else {
                visited[e0] = true;
                visited[e1] = true;
            }

            // a
            // |
            // v
            // b<---|
            // |    |
            // v    |
            // c ---
            // 形如这样的case, c-> b加的一根，这样形成一个小环
            // 此时b有两个parent，parent vector记录的是a，（假设从a开始走过来）
            // 然后e[0]是c，这样两个爹记录一下，一个是p1, 一个p2
            if parent[e1] > 0 {
                target = e1;
                p1 = parent[e1];
                p2 = e0;
            } else {
                parent[e1] = e0;
            }
        }
        // Remove any edge in the cycle is good, so remove the last one appeared in input.
        // The one node that has no parent is the root node. For the node that has two parents, we need to remove one edge there.
        // (a) If there is no cycle, then two parents are both from the root, so remove any edge is okay;
        // (b) If there is a cycle, then remove the cycle-side branch;
        if target == 0 {
            // no double father node, must has circle;
            // 也就是本身就是单个串起来，然后添加一根最后一个节点到root节点的边，形成一个包含所有元素的大环
            return cycle;
        } else {
            // if p1 is in a circle, then must remove p1 edge, otherwise, remove the
            // later one, p2
            // which means return the circle side edge
            while p1 != target && parent[p1] > 0 {
                p1 = parent[p1];
            }
            if p1 == target {
                return vec![parent[target] as i32, target as i32];
            }
            return vec![p2 as i32, target as i32];
        }
    }
}

fn main() {}
