#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut vals: Vec<i32> = (1..=m).collect();
        let mut ret: Vec<i32> = Vec::new();

        for q in queries.into_iter() {
            let idx = vals.iter().position(|&x| x == q).unwrap();
            ret.push(idx as i32);
            vals.remove(idx);
            vals.insert(0, q);
        }

        ret
    }
}

fn main() {}
