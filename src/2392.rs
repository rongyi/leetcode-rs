struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let k = k as usize;
        let rows = Self::top_sort(k, &row_conditions);
        let cols = Self::top_sort(k, &col_conditions);
        if rows.is_empty() || cols.is_empty() {
            return Vec::new();
        }
        let mut idx: Vec<(usize, usize)> = vec![(0, 0); k];
        for i in 0..k {
            idx[rows[i] - 1].0 = i;
            idx[cols[i] - 1].1 = i;
        }
        let mut ret: Vec<Vec<i32>> = vec![vec![0; k]; k];
        for i in 0..k {
            ret[idx[i].0][idx[i].1] = i as i32 + 1;
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

        if visited == k {
            return ret;
        }

        return vec![];
    }
}

fn main() {}
