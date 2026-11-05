struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        for &num in basket1.iter() {
            *cnt.entry(num).or_default() += 1;
        }
        for &num in basket2.iter() {
            *cnt.entry(num).or_default() -= 1;
        }
        let mut exch = vec![];
        for (&k, &v) in cnt.iter() {
            if v % 2 != 0 {
                return -1;
            }
            exch.extend(vec![k; (v.abs() / 2) as usize]);
        }
        exch.sort_unstable();
        exch.truncate(exch.len() / 2);
        let min_val = *basket1.iter().chain(basket2.iter()).min().unwrap();
        exch.iter()
            .fold(0i64, |acc, &cur| acc + cur.min(2 * min_val) as i64)
    }
}

fn main() {}
