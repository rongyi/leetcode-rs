struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut prevprev = 0;
                let mut prev = 1;
                for _ in 2..=n {
                    let next = prevprev + prev;
                    prevprev = prev;
                    prev = next;
                }

                prev
            }
        }
    }
}

fn main() {}
