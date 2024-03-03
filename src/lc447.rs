
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        for i in 0..points.len() {
            let mut distance_cnt = HashMap::new();

            for j in 0..points.len() {
                if i != j {
                    let cur_d = Self::calc_distance(&points[i], &points[j]);
                    *distance_cnt.entry(cur_d).or_insert(0) += 1;
                }
            }
            for &cnt in distance_cnt.values() {
                ret += cnt * (cnt - 1);
            }
        }

        ret
    }

    fn calc_distance(p1: &[i32], p2: &[i32]) -> i32 {
        let dx = p1[0] - p2[0];
        let dy = p1[1] - p2[1];
        dx * dx + dy * dy
    }
}

fn main() {}
