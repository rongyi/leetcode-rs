#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut odd_cnt = 0;
        let mut event_cnt = 0;
        for &p in position.iter() {
            if p % 2 == 0 {
                odd_cnt += 1;
            } else {
                event_cnt += 1;
            }
        }
        // if odd_cnt == 0 || event_cnt == 0 {
        //     return 0;
        // }

        odd_cnt.min(event_cnt)
    }
}

fn main() {}
