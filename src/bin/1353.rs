struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// 1353. Maximum Number of Events That Can Be Attended
///
/// Sweep forward in time. On any day, every event that has already started is a
/// candidate, and the greedy pick is the one that expires first (smallest end
/// day) — attending it leaves the longer-lived events for later days.
///
/// The only real change from a day-by-day simulation: when nothing is available
/// we jump `day` straight to the next start day, so the loop iterates over
/// events instead of over calendar days.
impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|e| e[0]);

        // min-heap of end days
        let mut ends: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut attended = 0;
        let mut day = 0;
        let mut i = 0;

        while i < events.len() || !ends.is_empty() {
            // Idle stretch: skip it in one step. (`i < len` is guaranteed here,
            // otherwise the loop condition would already be false.)
            if ends.is_empty() {
                day = day.max(events[i][0]);
            }

            // Everything that has started by `day` is available.
            while i < events.len() && events[i][0] <= day {
                ends.push(Reverse(events[i][1]));
                i += 1;
            }

            // Forget events that can no longer be attended.
            while let Some(&Reverse(end)) = ends.peek() {
                if end < day {
                    ends.pop();
                } else {
                    break;
                }
            }

            // One event per day: take the one ending soonest.
            if ends.pop().is_some() {
                attended += 1;
                day += 1;
            }
        }

        attended
    }
}

fn main() {
    let cases = [
        (vec![vec![1, 2], vec![2, 3], vec![3, 4]], 3),
        (vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]], 4),
        ((1..=7).map(|end| vec![1, end]).collect::<Vec<_>>(), 7),
        // only days 1 and 2 exist, so the third copy is lost
        (vec![vec![1, 2], vec![1, 2], vec![1, 2]], 2),
        // a long idle stretch between events must not cost anything
        (vec![vec![1, 1], vec![100_000, 100_000]], 2),
    ];

    for (events, want) in cases {
        let got = Solution::max_events(events.clone());
        println!("{events:?} -> {got} (want {want})");
        assert_eq!(got, want, "{events:?}");
    }

    // one event per day for 100k events sharing a start day
    let big: Vec<Vec<i32>> = (0..100_000).map(|i| vec![1, 100_000 - i]).collect();
    let got = Solution::max_events(big);
    println!("100k events -> {got} (want 100000)");
    assert_eq!(got, 100_000);

    println!("all cases passed");
}
