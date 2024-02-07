struct Solution;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph = HashMap::new();
        for ticket in tickets {
            graph
                .entry(ticket[0].clone())
                .or_insert(BinaryHeap::new())
                .push(Reverse(ticket[1].clone()));
        }
        let mut ret = Vec::new();
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

fn main() {
    // default to max heap
    let mut a = BinaryHeap::new();
    a.push(1);
    a.push(2);
    println!("{}", a.pop().unwrap());
}
