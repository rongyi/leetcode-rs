#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn distance_between_bus_stops(
        distance: Vec<i32>,
        mut start: i32,
        mut destination: i32,
    ) -> i32 {
        let mut circle: i64 = distance.iter().fold(0i64, |acc, cur| acc + *cur as i64);
        let mut part = 0;
        if start > destination {
            let tmp = start;
            start = destination;
            destination = tmp;
        }

        for i in start..destination {
            part += distance[i as usize] as i64;
        }

        part.min(circle - part) as i32
    }
}

fn main() {}
