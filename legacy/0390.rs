struct Solution;


impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut left = true;
        let mut remaining = n;
        let mut step = 1;
        let mut head = 1;

        while remaining > 1 {
            if left || remaining % 2 == 1 {
                head = head + step;
            }
            remaining /= 2;
            step *= 2;
            left = !left;
        }

        head
    }
}

fn main() {
}
