#![allow(dead_code)]


struct Solution;

use std::f64::consts::PI;
impl Solution {
    pub fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
        let mut ret = 1;
        let r = r as f64;
        for p in darts.iter() {
            let (x, y) = (p[0] as f64, p[1] as f64);
            let mut alpha = 0.0;
            while alpha < 360.0 {
                let cur_cos = (PI * alpha / 180.0).cos();
                let cur_sin = (PI * alpha / 180.0).sin();

                let cx = x + r * cur_cos;
                let cy = y + r * cur_sin;
                let mut cur_cnt = 0;
                for p2 in darts.iter() {
                    let dist = ((p2[0] as f64 - cx).powi(2) + ((p2[1] as f64 - cy).powi(2))).sqrt();
                    if dist - r < 0.01 {
                        cur_cnt += 1;
                    }
                }

                ret = ret.max(cur_cnt);
                alpha += 1.0;
            }
        }

        ret
    }
}

fn main() {}
