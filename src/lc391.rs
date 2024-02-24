struct Solution;
use std::collections::{HashMap, HashSet};


impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut area = 0;
        let mut xmin = i32::MAX;
        let mut ymin = i32::MAX;

        let mut xmax = i32::MIN;
        let mut ymax = i32::MIN;

        let mut m: HashMap<(i32, i32), i32> = HashMap::new();
        let points = [[0, 1], [0, 3], [2, 3], [2, 1]];
        for v in &rectangles {
            for d in &points {
                let e = m.entry((v[d[0]], v[d[1]])).or_insert(0);
                *e += 1;
                if *e > 4 {
                    return false;
                }
            }
            area += (v[3] - v[1]) * (v[2] - v[0]);
            xmin = xmin.min(v[0]);
            ymin = ymin.min(v[1]);
            xmax = xmax.max(v[2]);
            ymax = ymax.max(v[3]);
        }

        if area != (xmax - xmin) * (ymax - ymin) {
            return false;
        }
        let mut s: HashSet<(i32, i32)> = HashSet::new();
        s.insert((xmin, ymin));
        s.insert((xmin, ymax));
        s.insert((xmax, ymax));
        s.insert((xmax, ymin));

        for (pair, &cnt) in m.iter() {
            if cnt % 2 == 1 {
                if let Some(_p) = s.get(pair) {
                    s.remove(pair);
                } else {
                    // odd cnt can only be four cornes
                    return false;
                }
            }
        }

        s.is_empty()
    }
}

fn main() {}
