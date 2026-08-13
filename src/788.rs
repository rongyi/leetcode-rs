struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut acc = 0;

        for i in 1..=n {
            let s = i.to_string();
            // must not have 3 4 7
            if s.chars().any(|c| "347".find(c).is_some()) {
                continue;
            }
            // must have value not same
            if s.chars().any(|c| "2569".find(c).is_some()) {
                acc += 1;
            }
        }

        acc
    }
}

fn main() {}
