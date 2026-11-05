struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    /// LeetCode 815: Bus Routes — BFS over routes.
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let sz = routes.len();

        let mut bus_visited = vec![false; sz];
        let mut stop_visited = HashSet::new();
        let mut stop_buses: HashMap<i32, Vec<usize>> = HashMap::new();

        for (bus_id, stops) in routes.iter().enumerate() {
            for &stop in stops.iter() {
                stop_buses.entry(stop).or_default().push(bus_id);
            }
        }
        let mut q = VecDeque::new();
        for &bus in stop_buses.get(&source).unwrap_or(&vec![]) {
            if !bus_visited[bus] {
                bus_visited[bus] = true;
                q.push_back(bus);
            }
        }

        stop_visited.insert(source);
        let mut layers = 1;

        while !q.is_empty() {
            let layer_sz = q.len();
            for _ in 0..layer_sz {
                let cur_bus = q.pop_front().unwrap();
                for &stop in routes[cur_bus].iter() {
                    if stop == target {
                        return layers;
                    }
                    if stop_visited.insert(stop) {
                        for &next_bus in stop_buses.get(&stop).unwrap_or(&vec![]) {
                            if !bus_visited[next_bus] {
                                bus_visited[next_bus] = true;
                                q.push_back(next_bus);
                            }
                        }
                    }
                }
            }
            layers += 1;
        }

        -1
    }
}

/// Reference: BFS over stops (slow but obviously correct).
fn reference(routes: &[Vec<i32>], source: i32, target: i32) -> i32 {
    if source == target {
        return 0;
    }
    let mut stop_to_bus: HashMap<i32, Vec<usize>> = HashMap::new();
    for (b, r) in routes.iter().enumerate() {
        for &s in r {
            stop_to_bus.entry(s).or_default().push(b);
        }
    }
    // BFS over (stop, buses_taken), but to avoid huge states, BFS over stops
    // with bus-count as distance, allowing revisits only when strictly better.
    // Simpler: BFS where each node is (stop, bus_id) we rode to reach it.
    let mut best = i32::MAX;
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((source, 0));
    visited.insert(source);
    while let Some((stop, cost)) = q.pop_front() {
        if stop == target {
            best = best.min(cost);
            continue;
        }
        for &b in stop_to_bus.get(&stop).unwrap_or(&vec![]) {
            let mut reached: HashSet<i32> = HashSet::new();
            for &s in &routes[b] {
                if visited.contains(&s) {
                    continue;
                }
                if reached.insert(s) {
                    q.push_back((s, cost + 1));
                    visited.insert(s);
                }
            }
        }
    }
    if best == i32::MAX {
        -1
    } else {
        best
    }
}

fn main() {
    // Fixed tests.
    let tests = [
        (vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6, 2),
        (vec![vec![7, 12], vec![4, 5, 15], vec![6], vec![15, 19], vec![9, 12, 13]], 15, 12, -1),
        (vec![vec![1, 2], vec![2, 3], vec![3, 4]], 1, 4, 3),
    ];
    for (routes, source, target, expected) in &tests {
        let result = Solution::num_buses_to_destination(routes.clone(), *source, *target);
        assert_eq!(result, *expected, "fixed test failed");
    }
    println!("✓ fixed tests pass");

    // Random stress test vs reference.
    let mut seed = 12345u64;
    let mut rng = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (seed >> 33) as usize
    };

    let mut mismatches = 0;
    for _ in 0..20000 {
        let num_routes = 1 + rng() % 6;
        let mut routes = Vec::new();
        for _ in 0..num_routes {
            let len = 1 + rng() % 5;
            let mut stops = Vec::new();
            for _ in 0..len {
                stops.push((rng() % 12) as i32); // may repeat within a route
            }
            stops.sort();
            stops.dedup();
            routes.push(stops);
        }
        let source = (rng() % 12) as i32;
        let target = (rng() % 12) as i32;

        let mine = Solution::num_buses_to_destination(routes.clone(), source, target);
        let ref_ans = reference(&routes, source, target);
        if mine != ref_ans {
            mismatches += 1;
            println!("MISMATCH: routes={:?} {}→{} mine={} ref={}", routes, source, target, mine, ref_ans);
            if mismatches > 5 {
                break;
            }
        }
    }
    if mismatches == 0 {
        println!("✓ 20000 random cases match reference");
    }
}
