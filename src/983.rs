struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut ret = 0;

        let mut last7: VecDeque<(i32, i32)> = VecDeque::new();
        let mut last30: VecDeque<(i32, i32)> = VecDeque::new();
        for d in days.into_iter() {
            while !last7.is_empty() && last7.front().unwrap().0 + 7 <= d {
                last7.pop_front();
            }
            while !last30.is_empty() && last30.front().unwrap().0 + 30 <= d {
                last30.pop_front();
            }
            last7.push_back((d, ret + costs[1]));
            last30.push_back((d, ret + costs[2]));

            let val1 = ret + costs[0];
            let val2 = last7.front().unwrap().1.min(last30.front().unwrap().1);
            // i32::min(ret + costs[0], val);
            let val = val1.min(val2);
            ret = val;
        }

        ret
    }
}

fn main() {}
