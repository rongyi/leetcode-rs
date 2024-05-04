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
            //          |___|
            //      |_______|
            // 按照排序规则，结尾相同的，短的在前
            // 所以出现这种情况 [p1, p2]肯定满足
            if intervals[i][0] <= p1 {
                continue;
            }

            // |----| |-----|
            //   p1 p2 cur
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
