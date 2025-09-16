struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let running_costs: Vec<i64> = running_costs.into_iter().map(|i| i as i64).collect();
        let charge_times: Vec<i64> = charge_times.into_iter().map(|i| i as i64).collect();
        let mut q: VecDeque<i64> = VecDeque::new();
        let sz = charge_times.len();

        let mut i = 0;
        let mut acc = 0i64;
        for j in 0..sz {
            // equal can stay
            while !q.is_empty() && *q.back().unwrap() < charge_times[j] {
                q.pop_back();
            }
            q.push_back(charge_times[j]);
            acc += running_costs[j];

            if acc * (j - i + 1) as i64 + *q.front().unwrap() > budget {
                acc -= running_costs[i];
                // so the equal can out
                if *q.front().unwrap() == charge_times[i] {
                    q.pop_front();
                }
                i += 1;
            }
        }

        (sz - i) as _
    }
}

fn main() {}
