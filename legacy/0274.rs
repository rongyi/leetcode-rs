struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort();

        let n = citations.len() as i32;

        for i in 0..n {
            if citations[i as usize] >= n - i {
                return n - i;
            }
        }

        0
    }
}

fn main() {
    unimplemented!();
}
