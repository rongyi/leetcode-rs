struct Solution;

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let gap = n - 1;
        let round = time / gap;
        let rem = time % gap;

        if round % 2 == 1 {
            return n - rem;
        }
        1 + rem
    }
}

fn main() {}
