struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }

        if n < 5 {
            return n;
        }

        let mut ret = 2;
        let mut len = 2;
        let mut copy_len = 1;
        while len < n {
            if n % len == 0 {
                copy_len = len;
                ret += 2;
                len = len * 2;
            } else {
                len += copy_len;
                ret += 1;
            }
        }
        ret
    }
}

fn main() {}
