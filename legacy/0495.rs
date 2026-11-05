struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        // the longest range of attack
        let mut prev = -1;
        let mut ret = 0;
        for t in time_series.into_iter() {
            let cur_end = t + duration;
            ret += cur_end - t.max(prev);

            prev = prev.max(t + duration);
        }

        ret
    }
}

fn main() {}
