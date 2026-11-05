#![allow(dead_code)]


struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let sz = start_time.len();
        // start, end, profit
        let mut jobs: Vec<(i32, i32, i32)> = (0..sz)
            .map(|i| (start_time[i], end_time[i], profit[i]))
            .collect();
        // sort by ending time
        jobs.sort_by_key(|&(_, end, _)| end);
        // let's take first profit as granted
        let mut max_so_far = jobs[0].2;
        // endtime -> max_profit, easy to make us chain together
        let mut prev: BTreeMap<i32, i32> = BTreeMap::new();
        prev.insert(jobs[0].1, max_so_far);

        for i in 1..sz {
            let (start, end, profit) = jobs[i];
            let curr_profit = profit;
            // let's see we can chain prev or not
            let prev_profit = prev
                .range(0..=start)
                .next_back()
                .map_or(0, |(_, &profit)| profit);
            max_so_far = max_so_far.max(curr_profit + prev_profit);
            prev.insert(end, max_so_far);
        }

        max_so_far
    }
}

fn main() {}
