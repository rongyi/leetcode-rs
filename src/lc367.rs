struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        for i in 0.. {
            // overflow the case
            if i * i > num || i * i < 0 {
                break;
            }
            if i * i == num {
                return true;
            }
        }

        false
    }
}

fn main() {}
