struct Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // already sorted
        // intervals.sort_unstable();
        let sz = intervals.len();
        let mut i = 0;
        let mut fresh = vec![];
        let (mut target_start, mut target_end) = (new_interval[0], new_interval[1]);
        while i < sz {
            let (cur_start, cur_end) = (intervals[i][0], intervals[i][1]);

            // after i just
            if cur_start > target_end {
                break;
            }
            // pass
            if cur_end < target_start {
                fresh.push(vec![cur_start, cur_end]);
            } else {
                target_start = target_start.min(cur_start);
                target_end = target_end.max(cur_end);
            }

            i += 1;
        }
        fresh.push(vec![target_start, target_end]);
        fresh.extend(intervals[i..].iter().cloned());

        fresh
    }
}

fn main() {}
