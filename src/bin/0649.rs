struct Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut rq: VecDeque<usize> = VecDeque::new();
        let mut dq: VecDeque<usize> = VecDeque::new();
        let senate: Vec<char> = senate.chars().collect();
        let sz = senate.len();
        for (i, &c) in senate.iter().enumerate() {
            if c == 'R' {
                rq.push_back(i);
            } else {
                dq.push_back(i);
            }
        }
        while !rq.is_empty() && !dq.is_empty() {
            let cur_r = rq.pop_front().unwrap();
            let cur_d = dq.pop_front().unwrap();
            if cur_r < cur_d {
                rq.push_back(cur_r + sz);
            } else {
                dq.push_back(cur_d + sz);
            }
        }
        if rq.len() > dq.len() {
            return "Radiant".to_string();
        }

        "Dire".to_string()
    }
}

fn main() {}
