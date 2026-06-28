struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new();
        for t in &tickets {
            let (from, to) = (t[0].clone(), t[1].clone());
            graph
                .entry(from)
                .or_insert(BinaryHeap::new())
                .push(Reverse(to));
        }
        let mut stack = vec!["JFK".to_string()];
        let mut ret: Vec<String> = Vec::new();

        while let Some(top) = stack.last_mut() {
            if let Some(destinations) = graph.get_mut(top) {
                if let Some(Reverse(next)) = destinations.pop() {
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
