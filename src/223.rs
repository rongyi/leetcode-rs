struct Solution;

impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let sum = (c - a) * (d - b) + (g - e) * (h - f);

        // no inersection
        if e >= c || h <= b || f >= d || a >= g {
            return sum;
        }

        let mut x = vec![a, e, g, c];
        let mut y = vec![b, f, h, d];
        x.sort();
        y.sort();

        sum - (x[2] - x[1]) * (y[2] - y[1])
    }
}

fn main() {}
