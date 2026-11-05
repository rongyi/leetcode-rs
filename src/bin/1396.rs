#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;
struct UndergroundSystem {
    // id -> (station, ts)
    in_cache: HashMap<i32, (String, i32)>,
    // in+out Statio -> (intervals, cnt)
    out_cache: HashMap<String, (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Self {
            in_cache: HashMap::new(),
            out_cache: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.in_cache.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((in_name, start)) = self.in_cache.get(&id) {
            let key = in_name.clone() + "->" + &station_name;
            self.out_cache.entry(key.clone()).or_insert((0, 0)).0 += t - *start;
            self.out_cache.entry(key).or_insert((0, 0)).1 += 1;
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let key = start_station + "->" + &end_station;
        if let Some(val) = self.out_cache.get(&key) {
            val.0 as f64 / val.1 as f64
        } else {
            0.0
        }
    }
}

fn main() {}
