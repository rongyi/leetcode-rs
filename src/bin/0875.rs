struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();

        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::can_finish(&piles, h, mid) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn can_finish(piles: &[i32], h: i32, speed: i32) -> bool {
        let mut hours = 0;
        for &pile in piles {
            // Calculate hours needed for this pile
            // Ceiling division: (pile + speed - 1) / speed
            hours += (pile + speed - 1) / speed;
            // Early exit if already exceeded h
            if hours > h {
                return false;
            }
        }
        hours <= h
    }
}

fn main() {}
