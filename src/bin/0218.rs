struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points: Vec<(i32, i32)> = vec![];
        for b in buildings.iter() {
            let (start, end, height) = (b[0], b[1], b[2]);
            points.push((start, -height));
            points.push((end, height));
        }
        points.sort_unstable();
        // height -> count
        let mut height_count: BTreeMap<i32, i32> = BTreeMap::new();
        // height dance with initial value 0, and when height change, we record this mother fucker point
        height_count.insert(0, 1);

        let mut ret = vec![];

        let mut prev_height = 0;
        for &(x, y) in points.iter() {
            if y < 0 {
                *height_count.entry(-y).or_default() += 1;
            } else {
                height_count.entry(y).and_modify(|v| *v -= 1);
                if let Some(&v) = height_count.get(&y) {
                    if v == 0 {
                        height_count.remove(&y);
                    }
                }
            }
            let &max_height = height_count.iter().rev().next().unwrap().0;
            if prev_height != max_height {
                ret.push(vec![x, max_height]);
                prev_height = max_height;
            }
        }
        ret
    }
}

fn main() {}
