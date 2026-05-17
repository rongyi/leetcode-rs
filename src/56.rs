struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();
        let mut ret = vec![];
        let sz = intervals.len();

        let mut prev_start = intervals[0][0];
        let mut prev_end = intervals[0][1];
        let mut i = 1;
        while i < sz {
            let (cur_start, cur_end) = (intervals[i][0], intervals[i][1]);
            // there's a gap, we can make sure prev interval can be pushed
            if cur_start > prev_end {
                ret.push(vec![prev_start, prev_end]);
                prev_start = cur_start;
                prev_end = cur_end;
            } else {
                // consume current interval
                prev_end = prev_end.max(cur_end);
            }

            i += 1;
        }
        ret.push(vec![prev_start, prev_end]);

        ret
    }
}

fn main() {}
