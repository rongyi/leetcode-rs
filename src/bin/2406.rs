struct Solution;

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut line_sweep = vec![0; 1e6 as usize + 2];
        for itv in intervals.iter() {
            line_sweep[itv[0] as usize] += 1;
            line_sweep[itv[1] as usize + 1] -= 1;
        }
        let mut ret = 0;
        let mut acc = 0;
        for &v in line_sweep.iter() {
            acc += v;
            ret = ret.max(acc);
        }
        ret
    }
}

fn main() {}
