struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        for &t in tasks.iter() {
            *cnt.entry(t).or_default() += 1;
        }
        let mut total = 0;
        for (_, &v) in cnt.iter() {
            if v < 2 {
                return -1;
            }
            // let op1 = (v + 1) / 2;
            // always use 3 to divde
            let op2 = (v + 2) / 3;
            total += op2;
        }
        total
    }
}

fn main() {}
