struct Solution;

impl Solution {
    pub fn min_processing_time(mut processor_time: Vec<i32>, mut tasks: Vec<i32>) -> i32 {
        processor_time.sort_unstable();
        // in descending
        tasks.sort_by(|a, b| b.cmp(a));
        let mut ret = 0;

        for i in 0..processor_time.len() {
            let cur = processor_time[i] + tasks[i * 4];
            ret = ret.max(cur);
        }

        ret
    }
}

fn main() {}
