struct Solution;

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut ret = 0;
        let mut visited = vec![false; n as usize + 1];
        let mut cur: Vec<usize> = Vec::new();

        Self::dfs(n as usize, &mut cur, &mut visited, &mut ret);

        ret
    }

    fn dfs(n: usize, cur: &mut Vec<usize>, visited: &mut Vec<bool>, ret: &mut i32) {
        if cur.len() == n {
            *ret += 1;
            return;
        }
        let l = cur.len() + 1;
        for i in 1..=n {
            if !visited[i] && (l % i == 0 || i % l == 0) {
                visited[i] = true;
                cur.push(i);
                Self::dfs(n, cur, visited, ret);

                visited[i] = false;
                cur.pop();
            }
        }
    }
}

fn main() {}
