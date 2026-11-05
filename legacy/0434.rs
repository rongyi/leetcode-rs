struct Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let chunks: Vec<&str> = s.split(' ').into_iter().filter(|s| !s.is_empty()).collect();


        chunks.len() as i32
    }
}

fn main() {}
