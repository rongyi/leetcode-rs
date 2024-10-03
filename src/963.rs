struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        let sz = points.len();
        let mut centers = HashMap::new();

        for i in 0..sz {
            for j in i + 1..sz {
                let (x1, y1) = (points[i][0], points[i][1]);
                let (x2, y2) = (points[j][0], points[j][1]);
                let x = x1 + x2;
                let y = y1 + y2;
                centers
                    .entry((x, y))
                    .or_insert(Vec::new())
                    .push(vec![x1, y1, x2, y2]);
            }
        }
        let mut ret = 0;

        for value in centers.values() {
            let cur_sz = value.len();
            for i in 0..cur_sz {
                for j in i + 1..cur_sz {
                    let p1 = &value[i];
                    let p2 = &value[j];
                    if (p1[0] - p2[0]) * (p1[0] - p2[2]) + (p1[1] - p2[1]) * (p1[1] - p2[3]) == 0 {
                        let area = Self::distance(p1[0], p1[1], p2[0], p2[1])
                            * Self::distance(p1[0], p1[1], p2[2], p2[3]);
                        if ret == 0 || ret > area {
                            ret = area;
                        }
                    }
                }
            }
        }

        (ret as f64).sqrt()
    }

    fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i64 {
        (x1 - x2) as i64 * (x1 - x2) as i64 + (y1 - y2) as i64 * (y1 - y2) as i64
    }
}

fn main() {}
