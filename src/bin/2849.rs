struct Solution;

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let dx = (sx - fx).abs();
        let dy = (sy - fy).abs();

        let min_dist = dx.max(dy);
        if min_dist == 0 {
            return t != 1;
        }

        t >= min_dist
    }
}

fn main() {}
