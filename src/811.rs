#![allow(dead_code)]

struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut group: HashMap<String, i32> = HashMap::new();
        for cp in cpdomains.into_iter() {
            let vs: Vec<String> = cp.split(' ').map(|s| s.to_string()).collect();
            let val = vs[0].parse::<i32>().unwrap();
            let domains: Vec<String> = vs[1].split('.').map(|s| s.to_string()).collect();
            for i in 0..domains.len() {
                let key: Vec<String> = domains.iter().skip(i).map(|s| s.to_string()).collect();
                let key = key.join(".");
                *group.entry(key).or_insert(0) += val;
            }
        }
        let mut ret: Vec<String> = Vec::new();

        for (k, v) in group.into_iter() {
            let cur = format!("{} {}", v, k);
            ret.push(cur);
        }

        ret
    }
}

fn main() {}
