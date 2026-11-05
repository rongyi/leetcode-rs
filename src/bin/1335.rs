struct Solution;

/// 1335. Minimum Difficulty of a Job Schedule
///
/// One day at a time, keeping only the current row of the DP.
///
/// `dp[i]` = cheapest way to finish the first `i` jobs using the days seen so far.
/// Row 0 (one day) is just the prefix maximum; every further day tries all the
/// places today's block can start.
impl Solution {
    pub fn min_difficulty(jobs: Vec<i32>, d: i32) -> i32 {
        let sz = jobs.len();
        let d = d as usize;

        // At least one job per day.
        if d > sz {
            return -1;
        }

        // One day: the whole prefix is a single block, so it costs its max.
        let mut dp = vec![i32::MAX; sz + 1];
        let mut block_max = 0;
        for i in 1..=sz {
            block_max = block_max.max(jobs[i - 1]);
            dp[i] = block_max;
        }

        // Add one day at a time; `dp` stands for `days - 1` days when the
        // iteration starts.
        for days in 2..=d {
            let mut next = vec![i32::MAX; sz + 1];

            // Today's block is jobs[j..i]:
            //   j >= days - 1  -> the previous days-1 days can each take a job
            //   j <= i - 1     -> today is not idle
            for (i, slot) in next.iter_mut().enumerate().skip(days) {
                let mut today_max = 0;
                let mut best = i32::MAX;

                // Walk j downwards so today's max comes for free:
                // max(jobs[j..i]) = max(jobs[j], max(jobs[j+1..i]))
                for j in (days - 1..i).rev() {
                    today_max = today_max.max(jobs[j]);
                    best = best.min(dp[j] + today_max);
                }
                *slot = best;
            }

            dp = next;
        }

        dp[sz]
    }
}

fn main() {
    let cases = [
        (vec![6, 5, 4, 3, 2, 1], 2, 7),
        (vec![9, 9, 9], 4, -1),
        (vec![1, 1, 1], 3, 3),
        (vec![7, 1, 7, 1, 7, 1], 3, 15),
        (vec![11, 111, 22, 222, 33, 333, 44, 444], 6, 843),
    ];

    for (jobs, d, want) in cases {
        let got = Solution::min_difficulty(jobs.clone(), d);
        println!("jobs={jobs:?} d={d} -> {got} (want {want})");
        assert_eq!(got, want, "jobs={jobs:?} d={d}");
    }
    println!("all cases passed");
}
