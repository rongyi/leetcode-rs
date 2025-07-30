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
            let selected_tasks = tasks[..k].to_vec();
            let mut workers: VecDeque<i32> = workers[workers.len() - k..].to_vec().into();
            let mut rem_pills = pills;

            // match task from bigger to smaller
            for &t in selected_tasks.iter().rev() {
                if let Some(&backw) = workers.back() {
                    if backw >= t {
                        workers.pop_back();
                        continue;
                    }
                }
                if rem_pills <= 0 {
                    return false;
                }

                // ok, not consume this tasks, need pill
                if let Some(idx) = workers.iter().position(|&x| x + strength >= t) {
                    rem_pills -= 1;
                    workers.remove(idx);
                } else {
                    return false;
                }
            }

            true
        };

        let mut left = 0;
        let mut right = tasks.len().min(workers.len());
        let mut ret = 0;

        while left <= right {
            let mid = (left + right) / 2;
            if check(mid) {
                ret = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        ret as _
    }
}

fn main() {
    let tasks = vec![3, 2, 1];
    let workers = vec![0, 3, 3];
    Solution::max_task_assign(tasks, workers, 1, 1);
}
