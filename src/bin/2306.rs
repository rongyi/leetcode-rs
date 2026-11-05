struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut ret: i64 = 0;
        let mut groups: HashMap<u8, HashSet<Vec<char>>> = HashMap::new();
        for s in ideas.iter() {
            let first = s.chars().next().unwrap() as u8 - 'a' as u8;
            let rest: Vec<char> = s.chars().skip(1).collect();
            groups.entry(first).or_default().insert(rest);
        }

        for i in 0..25 {
            for j in i + 1..26 {
                match (groups.get(&i), groups.get(&j)) {
                    (None, _) | (_, None) => continue,
                    (Some(s1), Some(s2)) => {
                        let intersection_cnt = s1.intersection(s2).count() as i64;
                        ret += 2
                            * (s1.len() as i64 - intersection_cnt)
                            * (s2.len() as i64 - intersection_cnt);
                    }
                }
            }
        }

        ret
    }
}

fn main() {}
