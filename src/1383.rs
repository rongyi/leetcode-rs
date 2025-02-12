#![allow(dead_code)]
struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        // efficiency, speed
        let mut es: Vec<(i32, i32)> = efficiency.into_iter().zip(speed.into_iter()).collect();
        es.sort_unstable();

        let mut pq = BinaryHeap::new();

        let mut sum = 0i64;
        let mut ret = 0i64;

        for &(cur_eff, cur_speed) in es.iter().rev() {
            // sum of its engineers' speeds multiplied by the minimum efficiency
            sum += cur_speed as i64;
            pq.push(Reverse(cur_speed));
            if pq.len() > k as usize {
                sum -= pq.pop().unwrap().0 as i64;
            }

            ret = ret.max(sum * cur_eff as i64);
        }

        (ret % MOD) as i32
    }
}

fn main() {}
