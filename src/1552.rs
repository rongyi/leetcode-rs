#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort();

        let n = position.len();
        let mut left = 1;
        let mut right = position[n - 1] - position[0];

        while left <= right {
            let mid = left + (right - left) / 2;

            if Self::can_place(mid, &position, m) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        right as i32
    }

    fn can_place(dist: i32, position: &[i32], m: i32) -> bool {
        let mut count = 1;
        let mut last_pos = position[0];

        for i in 1..position.len() {
            if position[i] - last_pos >= dist {
                count += 1;
                last_pos = position[i];
                if count >= m {
                    return true;
                }
            }
        }

        count >= m
    }
}

fn main() {}
