#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let idx = match rule_key.as_ref() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => unreachable!(),
        };
        let mut ret = 0;
        for item in items.iter() {
            if item[idx] == rule_value {
                ret += 1;
            }
        }
        ret
    }
}

fn main() {}
