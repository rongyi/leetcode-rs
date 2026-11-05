struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut ret = Vec::new();
        let mut letter_logs = Vec::new();
        let mut digits_logs = Vec::new();
        for log in logs.iter() {
            let a = log.split(' ').skip(1).collect::<Vec<_>>();
            let is_all_digit = a
                .into_iter()
                .map(|s| s.chars().all(|c| c.is_ascii_digit()))
                .all(|c| c == true);
            if is_all_digit {
                digits_logs.push(log.clone());
            } else {
                letter_logs.push(log.clone());
            }
        }
        letter_logs.sort_by(|a, b| {
            let a_head = a.split(' ').next().unwrap();
            let b_head = b.split(' ').next().unwrap();
            let a_rest = a.split(' ').skip(1).collect::<Vec<_>>();
            let b_rest = b.split(' ').skip(1).collect::<Vec<_>>();
            if a_rest.cmp(&b_rest) == Ordering::Equal {
                a_head.cmp(&b_head)
            } else {
                a_rest.cmp(&b_rest)
            }
        });

        ret.extend(letter_logs);
        ret.extend(digits_logs);

        ret
    }
}

fn main() {}
