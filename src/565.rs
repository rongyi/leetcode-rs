struct Solution;

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut max_loop = 0;
        let mut visited = vec![false; nums.len()];
        for i in 0..nums.len() {
            if visited[i] {
                continue;
            }
            let mut cur_loop = 0;
            Self::dfs(&nums, i, &mut cur_loop, &mut max_loop, &mut visited);
        }

        max_loop
    }

    fn dfs(
        nums: &[i32],
        cur: usize,
        cur_loop: &mut i32,
        max_loop: &mut i32,
        visited: &mut Vec<bool>,
    ) {
        if visited[cur] {
            *max_loop = (*max_loop).max(*cur_loop);
            return;
        }
        visited[cur] = true;
        *cur_loop += 1;

        Self::dfs(nums, nums[cur] as usize, cur_loop, max_loop, visited);
    }
}

fn main() {}
