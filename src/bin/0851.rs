struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let sz = quiet.len();
        let mut rich_chain: HashMap<i32, Vec<i32>> = HashMap::new();

        for p in richer.iter() {
            rich_chain.entry(p[1]).or_default().push(p[0]);
        }

        let mut ret = vec![-1; sz];
        for i in 0..sz {
            Self::dfs(&rich_chain, i, &mut ret, &quiet);
        }

        ret
    }
    fn dfs(
        rich_chain: &HashMap<i32, Vec<i32>>,
        cur: usize,
        ret: &mut Vec<i32>,
        quite: &Vec<i32>,
    ) -> i32 {
        if ret[cur] >= 0 {
            return ret[cur];
        }
        ret[cur] = cur as i32;
        if let Some(neibs) = rich_chain.get(&(cur as i32)) {
            for &rich in neibs.iter() {
                if quite[ret[cur] as usize]
                    > quite[Self::dfs(rich_chain, rich as usize, ret, quite) as usize]
                {
                    ret[cur] = ret[rich as usize];
                }
            }
        }

        ret[cur]
    }
}

mod ai {
    struct Solution;

    use std::collections::VecDeque;

    impl Solution {
        pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
            let sz = quiet.len();
            let mut ret: Vec<i32> = (0..sz as i32).collect();

            // build graph, from richer to poorer
            let mut graph = vec![vec![]; sz];
            let mut indegree = vec![0; sz];

            for rich in richer.iter() {
                // u -> v
                let (u, v) = (rich[0] as usize, rich[1] as usize);
                graph[u].push(v);
                indegree[v] += 1;
            }
            let mut q = VecDeque::new();

            for i in 0..sz {
                if indegree[i] == 0 {
                    q.push_back(i);
                }
            }
            while let Some(u) = q.pop_front() {
                // to poorer node
                for &v in graph[u].iter() {
                    if quiet[ret[u] as usize] < quiet[ret[v] as usize] {
                        ret[v] = ret[u];
                    }
                    indegree[v] -= 1;
                    if indegree[v] == 0 {
                        q.push_back(v);
                    }
                }
            }

            ret
        }
    }
}

fn main() {}
