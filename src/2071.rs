struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        tasks.sort();
        workers.sort();

        let check = |k: usize| -> bool {
            let mut selected = tasks[..k].to_vec();
            let mut available: VecDeque<_> = workers[workers.len() - k..].to_vec().into();
            let mut rem_pills = pills;

            for task in selected.iter().rev() {
                if let Some(&w) = available.back() {
                    if w >= *task {
                        available.pop_back();
                        continue;
                    }
                }
                if rem_pills == 0 {
                    return false;
                }

                if let Some(idx) = available.iter().position(|&w| w + strength >= *task) {
                    available.remove(idx);
                    rem_pills -= 1;
                } else {
                    return false;
                }
            }
            true
        };

        let mut left = 0;
        let mut right = tasks.len().min(workers.len());

        while left < right {
            let mid = (left + right + 1) / 2;
            if check(mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left as _
    }
}

fn main() {}
