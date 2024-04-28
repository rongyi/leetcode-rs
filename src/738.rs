#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut mono: Vec<usize> = Vec::new();

        let mut ret = vec![0; temperatures.len()];
        for (i, &val) in temperatures.iter().enumerate().rev() {
            while !mono.is_empty() && val >= temperatures[*mono.last().unwrap()] {
                mono.pop();
            }

            if !mono.is_empty() {
                ret[i] = (*mono.last().unwrap() - i) as i32;
            }

            mono.push(i);
        }
        ret
    }
}

fn main() {}
