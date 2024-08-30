struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut workers: Vec<(f64, i32)> = quality
            .iter()
            .zip(wage.iter())
            .map(|(&q, &w)| (w as f64 / q as f64, q))
            .collect();

        workers.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut quality_sum = 0;
        let mut max_heap = BinaryHeap::new();
        let mut min_cost = f64::INFINITY;

        for (ratio, q) in workers {
            quality_sum += q;
            max_heap.push(q);

            if max_heap.len() > k as usize {
                // Anyone who has this question:
                // Why do we choose the highest quality person to remove? Why not choosing other workers?
                // Answer:
                // Since the workers are sorted in the increasing order of the wage/quality ratio, the global ratio will never decrease. For the previously scanned wrokers, we do not care about their personal ratios any more because their personal ratios will always be less than (or equal to) the current global ratio. So the previous workers' personal ratio will never affect the total payment.
                // Similarly, their personal base payment (i.e. the wage input) has been satisfied already. As the global ratio increases, their actual payemnt will only increase or stay the same, and will never become lower than their base payment.
                // So when deciding whom to remove, the only thing that matters is the workers' quality. With a given global ratio, removing the highest quality worker will reduces the total payment as much as possible. That is why we want to removing the highest quality worker.
                quality_sum -= max_heap.pop().unwrap();
            }

            if max_heap.len() == k as usize {
                min_cost = min_cost.min(ratio * quality_sum as f64);
            }
        }

        min_cost
    }
}

fn main() {}
