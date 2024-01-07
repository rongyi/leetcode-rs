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
            let x1 = points[i][0];
            let y1 = points[i][1];

            for j in (i + 1)..sz {
                let x2 = points[j][0];
                let y2 = points[j][1];

                let dx = x1 - x2;
                let dy = y1 - y2;
                let d = Self::gcd(dx, dy);

                let e = line.entry((dx / d, dy / d)).or_insert(0);
                *e += 1;
            }
            ret = ret.max(line.values().max().unwrap_or(&0) + 1);
        }

        ret
    }

    // fn gcd(a: i32, b: i32) -> i32 {
    //     if b == 0 {
    //         return a.abs();
    //     }
    //     gcd(b, a % b)
    // }

    fn gcd(a: i32, b: i32) -> i32 {
        let (mut x, mut y) = (a, b);
        while y != 0 {
            let temp = y;
            y = x % y;
            x = temp;
        }
        x
    }
}

fn main() {
    let p = vec![
        vec![1, 1],
        vec![3, 2],
        vec![5, 3],
        vec![4, 1],
        vec![2, 3],
        vec![1, 4],
    ];
    let i = Solution::max_points(p);
    println!("{}", i);
}
