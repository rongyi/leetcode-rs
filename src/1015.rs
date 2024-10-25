struct Solution;

impl Solution {
    // shitty
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut m = 1;
        let mut n = 1 % k;
        while n <= k {
            if m == 0 {
                return n;
            }

            m = (m * 10 + 1) % k;
            n += 1;
        }
        -1
    }
}

fn main() {}
