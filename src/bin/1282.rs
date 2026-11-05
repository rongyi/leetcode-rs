#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut group: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, &gid) in group_sizes.iter().enumerate() {
            let i = i as i32;
            group.entry(gid).or_default().push(i);
        }
        let mut ret = Vec::new();
        for (&gid, members) in group.iter() {
            let sz = members.len() as i32;
            let chunk = sz / gid;
            let mut start = 0;
            for _ in 0..chunk {
                let cur: Vec<i32> = members[start..start + gid as usize]
                    .iter()
                    .cloned()
                    .collect();
                ret.push(cur);
                start += gid as usize;
            }
        }
        ret
    }
}

fn main() {}
