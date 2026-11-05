#![allow(dead_code)]

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut ret = Vec::new();
        let mut group = HashMap::new();

        let mut txs = Vec::new();
        for (i, tx) in transactions.iter().enumerate() {
            let parts: Vec<_> = tx.split(',').collect();
            let name = parts[0].to_string();
            let time: i32 = parts[1].parse().unwrap();
            let amount: i32 = parts[2].parse().unwrap();
            let location = parts[3].to_string();
            let total = tx.clone();
            group.entry(name.clone()).or_insert(Vec::new()).push((
                i,
                name.clone(),
                time,
                amount,
                location.clone(),
                total.clone(),
            ));
            txs.push((name, time, amount, location, total));
        }
        for (i, tx) in txs.iter().enumerate() {
            if tx.2 > 1000 {
                ret.push(tx.4.clone());
            } else {
                for other_tx in group.get(&tx.0).unwrap() {
                    if i == other_tx.0 {
                        continue;
                    }
                    if other_tx.4 != tx.3 && (other_tx.2 - tx.1).abs() <= 60 {
                        ret.push(tx.4.clone());
                        // only once
                        break;
                    }
                }
            }
        }

        ret
    }
}

fn main() {}
