#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_time_required(mut jobs: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = jobs.len();

        // Sort jobs in descending order to assign bigger jobs first
        jobs.sort_unstable_by(|a, b| b.cmp(a));

        // Binary search for the result
        let mut left = jobs[0]; // Minimum possible answer is the largest job
        let mut right = jobs.iter().sum(); // Maximum is all jobs assigned to one worker

        while left < right {
            let mid = left + (right - left) / 2;
            if Self::is_possible(&jobs, k, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    // Check if it's possible to assign jobs to k workers with maximum time limit of limit
    fn is_possible(jobs: &[i32], k: usize, limit: i32) -> bool {
        let mut workers = vec![0; k];
        return Self::backtrack(jobs, &mut workers, 0, limit);
    }

    fn backtrack(jobs: &[i32], workers: &mut Vec<i32>, job_idx: usize, limit: i32) -> bool {
        if job_idx == jobs.len() {
            return true;
        }

        let mut seen = std::collections::HashSet::new();

        for i in 0..workers.len() {
            // Skip duplicate worker states
            if seen.contains(&workers[i]) {
                continue;
            }
            // accumulate work sum to cache, not current worker or job
            seen.insert(workers[i]);

            // Try to assign the current job to worker i
            if workers[i] + jobs[job_idx] <= limit {
                workers[i] += jobs[job_idx];
                if Self::backtrack(jobs, workers, job_idx + 1, limit) {
                    return true;
                }
                workers[i] -= jobs[job_idx];
            }

            // If this worker couldn't handle the job, and it's the first worker,
            // no need to try with other workers
            if workers[i] == 0 {
                break;
            }
        }

        false
    }
}

fn main() {}
