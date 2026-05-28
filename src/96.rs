struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        // empty
        dp[0] = 1;
        // only one node
        dp[1] = 1;

        // total nodes
        for nodes in 2..=n {
            // current root is root
            for root in 1..=nodes {
                // so there are root - 1 left total
                let left_child = root - 1;
                // nodes - root right total
                let right_child = nodes - root;
                dp[nodes] += dp[left_child] * dp[right_child];
            }
        }

        dp[n]
    }
}

fn main() {}
