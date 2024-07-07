#![allow(dead_code)]

struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        let mut ret = 0;
        let m = 1e9 as i64 + 7;
        arr.sort_unstable();
        let mut dp: HashMap<i32, i64> = HashMap::new();
        for i in 0..arr.len() {
            dp.insert(arr[i], 1);
            for j in 0..i {
                if arr[i] % arr[j] == 0 && dp.contains_key(&(arr[i] / arr[j])) {
                    let v1 = dp[&arr[j]];
                    let v2 = dp[&(arr[i] / arr[j])];
                    dp.entry(arr[i]).and_modify(|v| {
                        *v = (*v + v1 * v2) % m;
                    });
                }
            }
        }
        for (_, &v) in dp.iter() {
            ret = (ret + v) % m;
        }

        ret as i32
    }
}

fn main() {}
