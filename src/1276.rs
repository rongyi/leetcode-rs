#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if tomato_slices % 4 == 0 && tomato_slices / 4 == cheese_slices {
            return vec![tomato_slices / 4, 0];
        }
        for i in 0..cheese_slices {
            let small = (i + 1) * 2;
            let jumbo = tomato_slices - small;
            if small > tomato_slices {
                break;
            }
            if jumbo % 4 == 0 && jumbo / 4 == cheese_slices - (i + 1) {
                return vec![jumbo / 4, i + 1];
            }
        }
        vec![]
    }
}

fn main() {}
