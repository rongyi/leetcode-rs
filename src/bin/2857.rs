
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_pairs(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
        let mut ret = 0;

        for p in coordinates.iter() {
            let (x1, y1) = (p[0], p[1]);
            for x_dist in 0..=k {
                let y_dist = k - x_dist;

                let target_x = x1 ^ x_dist;
                let target_y = y1 ^ y_dist;
                if let Some(&count) = counts.get(&(target_x, target_y)) {
                    ret += count;
                }
            }
            *counts.entry((x1, y1)).or_insert(0) += 1;
        }
        ret
    }
}

fn main() {}
