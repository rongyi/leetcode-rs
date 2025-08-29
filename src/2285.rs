
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for r in roads.iter() {
            let (from, to) = (r[0], r[1]);
            graph.entry(from).or_default().push(to);
            graph.entry(to).or_default().push(from);
        }
        let mut total_score = 0;
        let mut edge_size = vec![];
        for v in graph.values() {
            edge_size.push(v.len());
        }
        edge_size.sort_by(|a, b| b.cmp(&a));
        let mut n = n as i64;
        for &e in edge_size.iter() {
            total_score += e as i64 * n;
            n -= 1;
        }

        total_score
    }
}

fn main() {}
