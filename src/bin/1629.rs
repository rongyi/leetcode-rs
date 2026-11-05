#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let chars: Vec<char> = keys_pressed.chars().collect();
        let mut longest_duration = release_times[0];
        let mut slowest_key = chars[0];

        for i in 1..release_times.len() {
            let duration = release_times[i] - release_times[i - 1];

            if duration > longest_duration
                || (duration == longest_duration && chars[i] > slowest_key)
            {
                longest_duration = duration;
                slowest_key = chars[i];
            }
        }

        slowest_key
    }
}

fn main() {}
