struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        //  A -> B
        //  -    +
        let mut degress: HashMap<i32, i32> = HashMap::new();
        // to -> from
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        for p in pairs.iter() {
            graph.entry(p[1]).or_default().push(p[0]);
            graph.entry(p[0]).or_default();
            *degress.entry(p[0]).or_default() -= 1;
            *degress.entry(p[1]).or_default() += 1;
        }

        let last = match degress.iter().find(|x| *x.1 == 1) {
            Some(v) => *v.0,
            // can be any node
            None => pairs[0][0],
        };

        Self::dfs(last, &mut graph, &mut ret);
        ret
    }

    fn dfs(cur: i32, graph: &mut HashMap<i32, Vec<i32>>, ret: &mut Vec<Vec<i32>>) {
        while let Some(prev) = graph.get_mut(&cur).unwrap().pop() {
            Self::dfs(prev, graph, ret);
            ret.push(vec![prev, cur]);
        }
    }
}

fn main() {
    let pairs = vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]];
}
