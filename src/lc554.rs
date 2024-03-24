struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut prefix_cnt: HashMap<i32, i32> = HashMap::new();
        let mut max_cnt = 0;

        for layer in wall.iter() {
            let mut prefix = 0;
            for (i, &w) in layer.iter().enumerate() {
                if i == layer.len() - 1 {
                    break;
                }
                prefix += w;
                let e = prefix_cnt.entry(prefix).or_insert(0);
                *e += 1;
                max_cnt = max_cnt.max(*e);
            }
        }

        wall.len() as i32 - max_cnt
    }
}

fn main() {}
