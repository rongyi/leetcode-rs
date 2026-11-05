struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut merged = Vec::new();
        let mut prev_interval = intervals[0].clone();

        for interval in intervals.iter().skip(0) {
            // current overlap with prev
            if interval[0] <= prev_interval[1] {
                prev_interval[1] = prev_interval[1].max(interval[1]);
            } else {
                merged.push(prev_interval.clone());
                prev_interval = interval.clone();
            }
        }
        merged.push(prev_interval);

        merged
    }
}
