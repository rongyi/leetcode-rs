struct Solution;

impl Solution {
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        let mut ret = 0;
        Self::dfs(1, &cost, n as usize, &mut ret);

        ret
    }
    fn dfs(idx: usize, cost: &[i32], n: usize, ret: &mut i32) -> i32 {
        if idx > n {
            return 0;
        }
        let l = Self::dfs(idx * 2, cost, n, ret);
        let r = Self::dfs(idx * 2 + 1, cost, n, ret);
        *ret += (l - r).abs();

        cost[idx - 1] + l.max(r)
    }
}

fn main() {}
