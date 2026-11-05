struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let k = k as usize;
        let row_order = Self::top_sort(k, &row_conditions);
        let col_order = Self::top_sort(k, &col_conditions);
        if row_order.is_empty() || col_order.is_empty() {
            return Vec::new();
        }
        // this way is better to understand
        let mut row_pos = vec![0; k + 1];
        for (i, &v) in row_order.iter().enumerate() {
            row_pos[v] = i;
        }
        let mut col_pos = vec![0; k + 1];
        for (i, &v) in col_order.iter().enumerate() {
            col_pos[v] = i;
        }
        let mut ret: Vec<Vec<i32>> = vec![vec![0; k]; k];
        for i in 1..=k {
            let r = row_pos[i];
            let c = col_pos[i];
            ret[r][c] = i as i32;
        }

        ret
    }
    fn top_sort(k: usize, conditions: &Vec<Vec<i32>>) -> Vec<usize> {
        let sz = k + 1;
        let mut indegree = vec![0; sz];
        let mut graph = vec![vec![]; sz];
        for e in conditions.iter() {
            let (from, to) = (e[0] as usize, e[1] as usize);
            graph[from].push(to);
            indegree[to] += 1;
        }
        let mut q = VecDeque::new();
        for i in 1..sz {
            if indegree[i] == 0 {
                q.push_back(i);
            }
        }
        let mut visited = 0;
        let mut ret = vec![];
        while let Some(cur) = q.pop_front() {
            visited += 1;
            ret.push(cur);
            for &v in graph[cur].iter() {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    q.push_back(v);
                }
            }
        }

        // means each one is on this chain, valid
        if visited == k {
            return ret;
        }

        return vec![];
    }
}

fn main() {}
