struct Solution;

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let sz = n as usize;
        let mut ret = vec![vec![]; sz];
        let mut graph: Vec<Vec<i32>> = vec![vec![]; sz];
        for e in edges.iter() {
            graph[e[0] as usize].push(e[1]);
        }
        for i in 0..n {
            Self::dfs(i, i, &graph, &mut ret);
        }

        ret
    }

    fn dfs(cur: i32, parent: i32, graph: &Vec<Vec<i32>>, ret: &mut Vec<Vec<i32>>) {
        for &c in graph[cur as usize].iter() {
            if ret[c as usize].is_empty() || *ret[c as usize].last().unwrap() != parent {
                ret[c as usize].push(parent);
                Self::dfs(c, parent, graph, ret);
            }
        }
    }
}

fn main() {}
