#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let sz = points.len();
        let mut ret = 0;

        for i in 1..sz {
            ret += (points[i][0] - points[i - 1][0])
                .abs()
                .max((points[i][1] - points[i - 1][1]).abs());
        }

        ret
    }
}
fn main() {}
