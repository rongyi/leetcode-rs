#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let dist = target[0].abs() + target[1].abs();
        for g in ghosts.iter() {
            let t = (g[0] - target[0]).abs() + (g[1] - target[1]).abs();
            if t <= dist {
                return false;
            }
        }

        true
    }
}

fn main() {}
