struct Solution;

impl Solution {
    pub fn prison_after_n_days(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut n = n % 14;
        if n == 0 {
            n = 14;
        }
        let mut next = vec![0; 8];
        for _ in 0..n {
            for i in 1..7 {
                next[i] = if cells[i + 1] == cells[i - 1] { 1 } else { 0 };
            }
            cells = next.clone();
        }

        next
    }
}

fn main() {}
