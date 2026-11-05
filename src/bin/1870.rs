#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        if (hour as usize) < dist.len() - 1
            || (hour as usize == dist.len() - 1 && hour.fract() == 0.0)
        {
            return -1;
        }

        let mut left = 1;
        let mut right = 10_000_000;

        while left < right {
            let mid = left + (right - left) / 2;

            if Self::can_finish(&dist, hour, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn can_finish(dist: &[i32], hour: f64, speed: i32) -> bool {
        let mut time = 0.0;

        for (i, &d) in dist.iter().enumerate() {
            let segment_time = d as f64 / speed as f64;
            time += if i == dist.len() - 1 {
                segment_time
            } else {
                segment_time.ceil()
            };
        }

        time <= hour
    }
}

fn main() {}
