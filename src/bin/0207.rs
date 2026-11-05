struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let sz = num_courses as usize;
        let mut visited = vec![false; sz];
        let mut in_degree = vec![0; sz];
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for pair in &prerequisites {
            // [0, 1] 1 -> 0
            in_degree[pair[0] as usize] += 1;
            graph.entry(pair[1]).or_default().push(pair[0]);
        }

        for i in 0..sz {
            if !visited[i] && in_degree[i] == 0 {
                Self::dfs(i, &graph, &mut visited, &mut in_degree);
            }
        }

        in_degree.into_iter().all(|x| x == 0)
    }

    fn dfs(
        cur: usize,
        graph: &HashMap<i32, Vec<i32>>,
        visited: &mut [bool],
        in_degree: &mut [i32],
    ) {
        visited[cur] = true;
        if let Some(nbs) = graph.get(&(cur as i32)) {
            for neib in nbs {
                in_degree[*neib as usize] -= 1;
                if !visited[*neib as usize] && in_degree[*neib as usize] == 0 {
                    Self::dfs(*neib as usize, graph, visited, in_degree);
                }
            }
        }
    }
}

fn main() {}
