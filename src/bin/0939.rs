struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let points: HashSet<(i32, i32)> = points.into_iter().map(|v| (v[0], v[1])).collect();
        let mut min_area = i32::MAX;
        let sz = points.len();
        for (x1, y1) in points.iter() {
            for (x2, y2) in points.iter() {
                if x1 != x2 && y1 != y2 {
                    if points.contains(&(*x1, *y2)) && points.contains(&(*x2, *y1)) {
                        let area = (x1 - x2).abs() * (y1 - y2).abs();
                        min_area = min_area.min(area);
                    }
                }
            }
        }
        if min_area == i32::MAX {
            return 0;
        }
        min_area
    }
}
fn main() {}
