struct Solution;

impl Solution {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        score.sort_by_key(|r| -r[k as usize]);

        score
    }
}

fn main() {}
