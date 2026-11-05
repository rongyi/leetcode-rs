#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut drink_total = 0;
        let mut empty_bottles = 0;
        while num_bottles > 0 {
            drink_total += num_bottles;
            // for next round
            num_bottles += empty_bottles;
            empty_bottles = num_bottles % num_exchange;
            num_bottles /= num_exchange;
        }
        drink_total
    }
}

fn main() {}
