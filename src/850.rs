#![allow(dead_code)]
struct Solution;

use std::collections::BTreeMap;

impl Solution {
    const MOD: i64 = 1_000_000_007;
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut sortx: BTreeMap<i32, Vec<(i32, i32, i32)>> = BTreeMap::new();
        for rec in rectangles.iter() {
            let (x1, y1, x2, y2) = (rec[0], rec[1], rec[2], rec[3]);
            sortx.entry(x1).or_default().push((y1, y2, 1));
            // when in x2, we need to cancel y1, y2 in accumulation
            // so -1 is a flag
            sortx.entry(x2).or_default().push((y1, y2, -1));
        }
        let mut y_acc: BTreeMap<i32, i32> = BTreeMap::new();

        let mut ret = 0;
        let mut prev_x = -1;
        for (&cur_x, y_dots) in sortx.iter() {
            // there's a valid gap between two x
            if prev_x >= 0 && cur_x - prev_x > 0 {
                let mut sum_y = 0;
                let mut sum = 0;
                let mut start = i32::MIN;

                for (&y, &cnt) in y_acc.iter() {
                    if cnt == 0 {
                        continue;
                    }
                    if start == i32::MIN {
                        start = y;
                    }
                    sum += cnt;
                    if sum == 0 {
                        sum_y += y - start;
                        start = i32::MIN;
                    }
                }
                ret += ((cur_x - prev_x) as i64 * sum_y as i64) % Self::MOD;
                ret %= Self::MOD;
            }

            for &(y1, y2, start_end) in y_dots.iter() {
                // when in rectangle start x
                // this will accumulate in a single line with y value
                // but when in end x
                // this will cancel the accumulation y1, y2 when in x2, i.e.
                // this rectangle is end
                *y_acc.entry(y1).or_default() += start_end;
                *y_acc.entry(y2).or_default() += -start_end;
            }

            prev_x = cur_x;
        }

        ret as i32
    }
}

fn main() {}
