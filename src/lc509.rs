struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut prevprev = 0;
        let mut prev = 1;
        let mut ret = 0;

        for _ in 2..=n {
            ret = prevprev + prev;
            prevprev = prev;
            prev = ret;
        }

        ret
    }
}

fn main() {}
