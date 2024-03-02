struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort();
        let mut ret = 0;
        let mut prev = 0;
        for i in 1..intervals.len() {
            // oh shit, we are covered by prev range!
            if intervals[i][0] < intervals[prev][1] {
                ret += 1;
                if intervals[i][1] < intervals[prev][1] {
                    prev = i;
                }
            } else {
                prev = i;
            }
        }

        ret
    }
}

fn main() {}
