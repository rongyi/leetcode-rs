#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|l, r| {
            if l[1] == r[1] {
                r[0].cmp(&l[0])
            } else {
                l[1].cmp(&r[1])
            }
        });
        let sz = intervals.len();

        let mut ret = 0;
        let mut p1 = -1;
        let mut p2 = -1;

        for i in 0..sz {
            if intervals[i][0] <= p1 {
                continue;
            }

            if intervals[i][0] > p2 {
                ret += 2;
                p2 = intervals[i][1];
                p1 = p2 - 1;
            } else {
                ret += 1;
                p1 = p2;
                p2 = intervals[i][1];
            }
        }

        ret
    }
}

fn main() {}
