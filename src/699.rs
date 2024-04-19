#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut h = 0;
        let mut ret = Vec::new();

        let mut intervals = Vec::new();

        for p in positions.iter() {
            let (start, end, side_len) = (p[0], p[0] + p[1] - 1, p[1]);

            h = h.max(Self::get_height(&mut intervals, (start, end, side_len)));
            ret.push(h);
        }

        ret
    }
    fn get_height(intervals: &mut Vec<(i32, i32, i32)>, mut cur: (i32, i32, i32)) -> i32 {
        let mut prev_height = 0;
        for itv in intervals.iter() {
            if itv.1 < cur.0 {
                continue;
            }
            if itv.0 > cur.1 {
                continue;
            }
            prev_height = prev_height.max(itv.2);
        }
        cur.2 += prev_height;
        intervals.push(cur);

        cur.2
    }
}

fn main() {}
