pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut cnt: HashMap<String, i32> = HashMap::new();
        for s in arr.iter() {
            *cnt.entry(s.to_string()).or_default() += 1;
        }
        let mut cur_uniq = 0;
        for s in arr.iter() {
            if *cnt.get(s).unwrap() == 1 {
                cur_uniq += 1;
                if cur_uniq == k {
                    return s.to_string();
                }
            }
        }
        "".to_string()
    }
}

fn main() {}
