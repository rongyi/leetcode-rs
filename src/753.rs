#![allow(dead_code)]
struct Solution;

impl Solution {
    // 懵逼
    pub fn crack_safe(n: i32, k: i32) -> String {
        let len = k.pow(n as u32 - 1);

        let mut visited = vec![vec![false; k as usize]; len as usize];

        let mut ret = String::new();

        Self::dfs(&mut visited, &mut ret, len, k, 0);

        let s = "0".repeat((n - 1) as usize);
        ret.push_str(&s);

        ret
    }

    fn dfs(visited: &mut Vec<Vec<bool>>, ret: &mut String, len: i32, k: i32, node: i32) {
        for i in 0..k {
            if !visited[node as usize][i as usize] {
                visited[node as usize][i as usize] = true;
                Self::dfs(visited, ret, len, k, (node * k + i) % len);
                ret.push(('0' as u8 + i as u8) as char);
            }
        }
    }
}

fn main() {}
