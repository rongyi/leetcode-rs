#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        // brute force
        let mut ret: f64 = 0.0;
        let sz = points.len();
        for i in 0..sz {
            for j in i + 1..sz {
                for k in j + 1..sz {
                    let a = Self::distance(&points[i], &points[j]);
                    let b = Self::distance(&points[i], &points[k]);
                    let c = Self::distance(&points[k], &points[j]);
                    if Self::valid_triangle(a, b, c) {
                        let s = (a + b + c) / 2.0;
                        let s = s * (s - a) * (s - b) * (s - c);
                        let s = s.sqrt();
                        ret = ret.max(s);
                    }
                }
            }
        }

        ret
    }
    fn distance(p1: &[i32], p2: &[i32]) -> f64 {
        let x = p1[0] - p2[0];
        let y = p1[1] - p2[1];
        let f = (x * x + y * y) as f64;

        f.sqrt()
    }
    fn valid_triangle(a: f64, b: f64, c: f64) -> bool {
        (a + b) > c && (b + c) > a && (a + c) > b
    }
}

fn main() {}
