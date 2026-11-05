struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let sz = points.len();
        if sz < 3 {
            return sz as i32;
        }

        let mut ret = 1;

        for i in 0..sz {
            let mut line = HashMap::new();
            let (x1, y1) = (points[i][0], points[i][1]);

            for j in (i + 1)..sz {
                let (x2, y2) = (points[j][0], points[j][1]);

                let dx = x1 - x2;
                let dy = y1 - y2;
                let d = Self::gcd(dx, dy);

                *line.entry((dx / d, dy / d)).or_insert(0) += 1;
            }
            ret = ret.max(line.values().max().unwrap_or(&0) + 1);
        }

        ret
    }

    fn gcd(mut x: i32, mut y: i32) -> i32 {
        while y != 0 {
            let v = x % y;
            x = y;
            y = v;
        }
        x
    }
}

fn main() {
    let v = Solution::gcd(-5, -30);
    println!("{}", v);
}
