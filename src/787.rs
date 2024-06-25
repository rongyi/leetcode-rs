#![allow(dead_code)]
struct Solution;

impl Solution {
    // bellman-ford
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut dist = vec![i32::MAX; n as usize];
        dist[src as usize] = 0;

        for i in 0..=k {
            let mut tmp = dist.clone();
            for flight in flights.iter() {
                let (from, to, weight) = (flight[0], flight[1], flight[2]);
                // ok, can use this airport
                if dist[from as usize] != i32::MAX {
                    tmp[to as usize] = tmp[to as usize].min(dist[from as usize] + weight);
                }
            }

            dist = tmp;
        }

        if dist[dst as usize] == i32::MAX {
            return -1;
        }
        dist[dst as usize]
    }
}

fn main() {}
