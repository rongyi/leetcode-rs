struct Solution;

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        (1..=n)
            .filter(|&num| num % 3 == 0 || num % 5 == 0 || num % 7 == 0)
            .sum()
    }
}

fn main() {}
