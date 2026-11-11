struct Solution;

/// 1361. Validate Binary Tree Nodes
///
/// A labelled structure is a valid binary tree exactly when:
///
///   1. every node has at most one parent   — no child is shared or duplicated,
///   2. exactly one node has no parent      — that is the root,
///   3. one walk down from the root reaches all `n` nodes — no cycle, no island.
///
/// Each condition is local and O(n), so the whole check is O(n).
impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let n = n as usize;

        // 1. count parents. A node has at most one left and one right owner, so
        //    `u8` is enough, and a second parent means it is not a tree.
        let mut parents = vec![0u8; n];
        for &child in left_child.iter().chain(right_child.iter()) {
            if child == -1 {
                continue;
            }
            let child = child as usize;
            // 任意一个 node 最多有一个爹
            parents[child] += 1;
            if parents[child] > 1 {
                return false;
            }
        }

        // 2. exactly one node must be parentless.
        let mut root = usize::MAX;
        for (node, &count) in parents.iter().enumerate() {
            if count == 0 {
                if root != usize::MAX {
                    return false; // two roots: a forest, not a tree
                }
                root = node;
            }
        }
        // 打环
        if root == usize::MAX {
            return false; // every node has a parent: the structure is all cycle
        }

        // 3. walk down from the root and count what is reachable. DFS order is
        //    irrelevant here, so a stack is cheaper than a queue.
        let mut stack = vec![root];
        let mut seen = vec![false; n];
        seen[root] = true;
        let mut visited = 0;

        while let Some(node) = stack.pop() {
            visited += 1;
            for child in [left_child[node], right_child[node]] {
                if child == -1 {
                    continue;
                }
                let child = child as usize;
                // Defensive: with (1) and (2) already enforced this cannot fire
                // (reaching a node twice would mean it has two parents), but it
                // makes the "never visit twice" invariant explicit.
                // if seen[child] {
                //     return false;
                // }
                seen[child] = true;
                stack.push(child);
            }
        }

        visited == n
    }
}

fn main() {
    let cases = [
        (
            "valid tree",
            4,
            vec![1, -1, 3, -1],
            vec![2, -1, -1, -1],
            true,
        ),
        (
            "node 3 has two parents",
            4,
            vec![1, -1, 3, -1],
            vec![2, 3, -1, -1],
            false,
        ),
        ("cycle, no root", 2, vec![1, 0], vec![-1, -1], false),
        (
            "two components",
            6,
            vec![1, -1, -1, 4, -1, -1],
            vec![2, -1, -1, 5, -1, -1],
            false,
        ),
        ("single node", 1, vec![-1], vec![-1], true),
        (
            "left and right are the same node",
            3,
            vec![1, -1, -1],
            vec![1, -1, -1],
            false,
        ),
    ];

    for (name, n, left, right, want) in cases {
        let got = Solution::validate_binary_tree_nodes(n, left, right);
        println!("{name:34} -> {got} (want {want})");
        assert_eq!(got, want, "{name}");
    }
    println!("all cases passed");
}
