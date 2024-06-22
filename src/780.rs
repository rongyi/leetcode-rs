#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        if tx < sx || ty < sy {
            return false;
        }

        if tx == sx && (ty - sy) % sx == 0 {
            return true;
        }
        if ty == sy && (tx - sx) % sy == 0 {
            return true;
        }

        Self::reaching_points(sx, sy, tx % ty, ty % tx)
    }
}

fn main() {}
