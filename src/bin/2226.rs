struct Solution;

impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut l = 0;
        let mut r = 1e7 as i64;
        while l < r {
            let m = (l + r + 1) / 2;
            let mut cnt = 0i64;
            for &c in candies.iter() {
                if cnt >= k {
                    break;
                }
                cnt += c as i64 / m;
            }
            if cnt >= k {
                l = m;
            } else {
                r = m - 1;
            }
        }
        l as _
    }
}

fn main() {}
