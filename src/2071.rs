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
