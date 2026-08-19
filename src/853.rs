struct Solution;

impl Solution {
    /// LeetCode 853: Car Fleet
    ///
    /// Cars catch up bumper-to-bumper and never pass. Count fleets at target.
    ///
    /// # Approach
    ///
    /// 1. arrival_time[i] = (target - position[i]) / speed[i]
    /// 2. Sort cars by position, CLOSEST to target first.
    /// 3. Scan closest → farthest. A car forms a NEW fleet iff its arrival
    ///    time is greater than the current fleet's arrival time (it can't
    ///    catch up). Otherwise it merges into the fleet ahead.
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, f64)> = position
            .iter()
            .zip(speed.iter())
            .map(|(&p, &s)| (p, (target - p) as f64 / s as f64))
            .collect();

        // Closest to target first.
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut fleets = 0;
        let mut fleet_time = 0.0; // arrival time of the slowest car so far

        for (_, time) in cars {
            if time > fleet_time {
                // Can't catch the fleet ahead → starts its own fleet.
                fleets += 1;
                fleet_time = time;
            }
            // else: catches up and merges — same fleet.
        }

        fleets
    }
}

fn main() {
    let tests = [
        (12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3], 3),
        (10, vec![3], vec![3], 1),
        (100, vec![0, 2, 4], vec![4, 2, 1], 1),
        (10, vec![6, 8], vec![3, 2], 2),
    ];

    for (target, position, speed, expected) in &tests {
        let result = Solution::car_fleet(*target, position.clone(), speed.clone());
        println!(
            "{} target={} position={:?} speed={:?} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            target,
            position,
            speed,
            result,
            expected
        );
    }
}
