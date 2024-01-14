struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let sz = num_courses as usize;
        let mut in_degree = vec![0; sz];
        let mut neibs: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut visited = vec![false; sz];
        for pair in &prerequisites {
            in_degree[pair[0] as usize] += 1;
            neibs.entry(pair[1]).or_default().push(pair[0]);
        }
        for i in 0..sz {
            if in_degree[i] == 0 && !visited[i] {
                Self::dfs(&mut visited, &neibs, &mut in_degree, i as i32);
            }
        }

        in_degree.into_iter().all(|x| x == 0)
    }

    fn dfs(visited: &mut [bool], neibs: &HashMap<i32, Vec<i32>>, in_degree: &mut [i32], cur: i32) {
        visited[cur as usize] = true;
        if let Some(nbs) = neibs.get(&cur) {
            for neib in nbs {
                in_degree[*neib as usize] -= 1;
                if !visited[*neib as usize] && in_degree[*neib as usize] == 0 {
                    Self::dfs(visited, neibs, in_degree, *neib);
                }
            }
        }
    }
}

fn main() {
    let input = vec![vec![1, 0]];
    Solution::can_finish(2, input);
}
