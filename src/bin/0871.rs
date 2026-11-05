struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut fuel = start_fuel as i64;
        let mut stops = 0;
        let mut prev_pos = 0;
        let mut max_heap = BinaryHeap::new();

        // Iterate through stations and also consider the target
        for station in stations.iter().chain(std::iter::once(&vec![target, 0])) {
            let position = station[0] as i64;
            let gas = station[1] as i64;

            // Need to travel this distance
            let distance = position - prev_pos;

            // If we can't reach this station, refuel using previous stations
            while fuel < distance {
                if max_heap.is_empty() {
                    return -1;
                }
                fuel += max_heap.pop().unwrap();
                stops += 1;
            }

            // We reached this station
            fuel -= distance;
            prev_pos = position;

            // Add this station's fuel as an option for future refueling
            if gas > 0 {
                max_heap.push(gas);
            }
        }

        stops
    }
}

fn main() {}
