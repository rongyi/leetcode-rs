#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        horizontal_cuts.sort_unstable();
        vertical_cuts.sort_unstable();
        let mut max_ho = horizontal_cuts[0].max(h - *horizontal_cuts.last().unwrap());
        let mut max_ver = vertical_cuts[0].max(w - *vertical_cuts.last().unwrap());

        for i in 1..horizontal_cuts.len() {
            max_ho = max_ho.max(horizontal_cuts[i] - horizontal_cuts[i - 1]);
        }
        for j in 1..vertical_cuts.len() {
            max_ver = max_ver.max(vertical_cuts[j] - vertical_cuts[j - 1]);
        }

        ((max_ho as i64) * (max_ver as i64) % (1e9 as i64 + 7)) as _
    }
}
fn main() {}
