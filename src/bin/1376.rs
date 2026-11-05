#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn num_of_minutes(_n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut relations: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, &leader) in manager.iter().enumerate() {
            if leader != -1 {
                relations.entry(leader as usize).or_default().push(i);
            }
        }

        Self::dfs(head_id as usize, &relations, &inform_time)
    }
    fn dfs(cur_id: usize, relations: &HashMap<usize, Vec<usize>>, inform_time: &Vec<i32>) -> i32 {
        let mut next_max = 0;

        if let Some(lst) = relations.get(&cur_id) {
            for &next_id in lst.iter() {
                next_max = next_max.max(Self::dfs(next_id, relations, inform_time));
            }
        }

        inform_time[cur_id as usize] + next_max
    }
}

fn main() {}
