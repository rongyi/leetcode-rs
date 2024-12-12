#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        arr2.sort_unstable();
        arr2.dedup();

        let mut dp: HashMap<(usize, i32), i32> = HashMap::new();
        let ret = Self::dfs(0, -1, &arr1, &arr2, &mut dp);
        if ret >= 10001 {
            -1
        } else {
            ret
        }
    }

    fn dfs(
        pos: usize,
        prev: i32,
        arr1: &[i32],
        arr2: &[i32],
        dp: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if pos == arr1.len() {
            return 0;
        }

        if let Some(&cache) = dp.get(&(pos, prev)) {
            return cache;
        }

        let mut cost = 10001;

        // ok, this is a valid order, so in one way, we can leave this number and keep going
        if arr1[pos] > prev {
            cost = cost.min(Self::dfs(pos + 1, prev, arr1, arr2, dp));
        }

        if let Some(&next) = arr2.iter().find(|&&x| x > prev) {
            cost = cost.min(1 + Self::dfs(pos + 1, next, arr1, arr2, dp));
        }

        dp.insert((pos, prev), cost);

        cost
    }
}

fn main() {
    let lst = vec![1, 2, 7, 9];
    let pos = lst.partition_point(|&x| x <= 10);
    println!("{pos}");
}
