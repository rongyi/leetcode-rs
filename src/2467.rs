struct Solution;

use std::usize;
impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, mut amount: Vec<i32>) -> i32 {
        let sz = amount.len();
        let mut parents: Vec<usize> = vec![usize::MAX; sz];
        let mut graph: Vec<Vec<usize>> = vec![vec![]; sz];

        for e in edges.iter() {
            let (from, to) = (e[0] as usize, e[1] as usize);
            graph[from].push(to);
            graph[to].push(from);
        }
        let mut parent_of = vec![usize::MAX; sz];
        let mut dist: Vec<usize> = vec![0; sz];
        Self::dfs(0, usize::MAX, 0, &mut parent_of, &mut dist, &graph);

        let mut cur = bob as usize;
        let mut bob_distance = 0;
        while parent_of[cur] != usize::MAX {
            if dist[cur] > bob_distance {
                amount[cur] = 0;
            } else if dist[cur] == bob_distance {
                amount[cur] /= 2;
            }
            cur = parent_of[cur];
            bob_distance += 1;
        }

        Self::max_sum(0, usize::MAX, &graph, &amount)
    }

    fn max_sum(cur: usize, parent: usize, graph: &Vec<Vec<usize>>, amount: &Vec<i32>) -> i32 {
        let mut ret = amount[cur];
        let mut sub_sum = i32::MIN;
        for &neib in graph[cur].iter() {
            if neib == parent {
                continue;
            }
            sub_sum = sub_sum.max(Self::max_sum(neib, cur, graph, amount));
        }
        if sub_sum != i32::MIN {
            ret += sub_sum;
        }
        ret
    }

    fn dfs(
        cur: usize,
        parent: usize,
        height: usize,
        parent_of: &mut Vec<usize>,
        dist_of: &mut Vec<usize>,
        graph: &Vec<Vec<usize>>,
    ) {
        parent_of[cur] = parent;
        dist_of[cur] = height;
        for &neib in graph[cur].iter() {
            if neib == parent {
                continue;
            }
            Self::dfs(neib, cur, height + 1, parent_of, dist_of, graph);
        }
    }
}

fn main() {}
