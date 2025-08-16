struct Solution;

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let sz = n as usize;
        let mut revs = vec![vec![]; sz];
        for e in edges.iter() {
            let (from, to) = (e[0] as usize, e[1] as usize);
            revs[to].push(from);
        }
        let mut ret = vec![];
        for i in 0..sz {
            let mut visited = vec![false; sz];
            let mut stack = vec![i];
            while let Some(cur) = stack.pop() {
                for &p in revs[cur].iter() {
                    if visited[p] {
                        continue;
                    }
                    visited[p] = true;
                    stack.push(p);
                }
            }
            // in this line all visited node is my ancestor
            let parents: Vec<i32> = (0..n).filter(|&v| visited[v as usize]).collect();
            ret.push(parents);
        }

        ret
    }
}
// method 2
// impl Solution {
//     pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         let sz = n as usize;
//         let mut ret = vec![vec![]; sz];
//         let mut graph: Vec<Vec<i32>> = vec![vec![]; sz];
//         for e in edges.iter() {
//             graph[e[0] as usize].push(e[1]);
//         }
//         for i in 0..n {
//             Self::dfs(i, i, &graph, &mut ret);
//         }

//         ret
//     }

//     fn dfs(cur: i32, parent: i32, graph: &Vec<Vec<i32>>, ret: &mut Vec<Vec<i32>>) {
//         for &c in graph[cur as usize].iter() {
//             if ret[c as usize].is_empty() || *ret[c as usize].last().unwrap() != parent {
//                 ret[c as usize].push(parent);
//                 Self::dfs(c, parent, graph, ret);
//             }
//         }
//     }
// }

fn main() {}
