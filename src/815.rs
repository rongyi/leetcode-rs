#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};
struct Solution;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let mut max_stop = -1;
        let mut stops: HashSet<i32> = HashSet::new();
        for r in routes.iter() {
            for &stop in r.iter() {
                max_stop = max_stop.max(stop);
                stops.insert(stop);
            }
        }
        // no way
        if !stops.contains(&source) || !stops.contains(&target) {
            return -1;
        }

        let mut min_to_reach = vec![i32::MAX; max_stop as usize + 1];
        min_to_reach[source as usize] = 0;

        let mut flag = true;
        let sz = routes.len() as i32;
        while flag {
            flag = false;
            for route in routes.iter() {
                let mut min_step = sz + 1;
                for &stop in route.iter() {
                    min_step = min_step.min(min_to_reach[stop as usize]);
                }
                min_step += 1;

                for &stop in route.iter() {
                    if min_to_reach[stop as usize] > min_step {
                        min_to_reach[stop as usize] = min_step;
                        flag = true;
                    }
                }
            }
        }
        if min_to_reach[target as usize] < sz + 1 {
            return min_to_reach[target as usize];
        }

        -1
    }
}

struct SolutionTLE;

impl SolutionTLE {
    pub fn num_buses_to_destinationTLENow(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        // group station -> buses
        let mut station: HashMap<i32, HashSet<usize>> = HashMap::new();
        for i in 0..routes.len() {
            for &cur_station in routes[i].iter() {
                // 该站台停靠i路公交车
                station.entry(cur_station).or_default().insert(i);
            }
        }
        let mut bfs: VecDeque<(i32, i32)> = VecDeque::new();
        bfs.push_back((source, 0));
        let mut visted: HashSet<i32> = HashSet::new();

        while !bfs.is_empty() {
            let (cur_station, cur_transfer) = bfs.pop_front().unwrap();
            if cur_station == target {
                return cur_transfer;
            }

            // 停靠此站台的所有公交车
            for &bus in station.get(&cur_station).unwrap_or(&HashSet::new()).iter() {
                // 几路公交车
                for &next_stop in routes[bus].iter() {
                    if !visted.contains(&next_stop) {
                        visted.insert(next_stop);
                        bfs.push_back((next_stop, cur_transfer + 1));
                    }
                }
            }
        }

        -1
    }
}

fn main() {}
