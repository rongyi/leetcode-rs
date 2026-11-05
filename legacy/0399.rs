struct Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let graph = Self::build_graph(&equations, &values);
        let mut ret = Vec::new();

        for q in queries.iter() {
            let mut visited = HashSet::new();
            let up = q[0].clone();
            let down = q[1].clone();
            let tmp = Self::get_weight(&graph, &up, &down, &mut visited);
            if tmp.is_some() {
                ret.push(tmp.unwrap());
            } else {
                ret.push(-1.0);
            }
        }

        ret
    }

    fn get_weight(
        graph: &HashMap<String, HashMap<String, f64>>,
        up: &str,
        down: &str,
        visited: &mut HashSet<String>,
    ) -> Option<f64> {
        if let Some(upneibs) = graph.get(up) {
            if let Some(val) = upneibs.get(down) {
                return Some(*val);
            }
            // multiply to get a / b * (b / c) -> a/c
            for (k, v) in upneibs.iter() {
                if !visited.contains(k) {
                    visited.insert(k.clone());
                    let val = Self::get_weight(graph, k, down, visited);
                    if val.is_some() {
                        return Some(*v * val.unwrap());
                    }
                }
            }

            None
        } else {
            None
        }
    }

    fn build_graph(
        equations: &Vec<Vec<String>>,
        values: &Vec<f64>,
    ) -> HashMap<String, HashMap<String, f64>> {
        let mut ret: HashMap<String, HashMap<String, f64>> = HashMap::new();

        for (eqt, &weight) in equations.iter().zip(values.iter()) {
            let start = eqt[0].clone();
            let end = eqt[1].clone();

            let e = ret.entry(start.clone()).or_insert(HashMap::new());
            e.insert(end.clone(), weight);

            let re = ret.entry(end).or_insert(HashMap::new());
            re.insert(start, 1.0 / weight);
        }

        ret
    }
}
fn main() {}
