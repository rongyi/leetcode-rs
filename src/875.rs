struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = 1000_000_000;
        while l < r {
            let m = l + (r - l) / 2;
            let mut total = 0;
            for &p in piles.iter() {
                total += (p + m - 1) / m;
            }
            if total > h {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}

fn main() {}
