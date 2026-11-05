struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new();
        let mut ret: Vec<String> = Vec::new();
        for t in &tickets {
            graph
                .entry(t[0].clone())
                .or_insert(BinaryHeap::new())
                .push(Reverse(t[1].clone()));
        }
        // using stack to expand the recursive call to eliminate the write exclusive problem
        let mut stack = vec!["JFK".to_string()];

        while let Some(top) = stack.last_mut() {
            if let Some(destination) = graph.get_mut(top) {
                if let Some(Reverse(next)) = destination.pop() {
                    stack.push(next);
                } else {
                    ret.push(stack.pop().unwrap());
                }
            } else {
                ret.push(stack.pop().unwrap());
            }
        }

        ret.reverse();
        ret
    }
}

fn main() {}
