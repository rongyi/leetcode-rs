#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|lst| lst.iter().sum()).max().unwrap()
    }
}

fn main() {}
