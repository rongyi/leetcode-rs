struct Solution;

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut sq = (area as f64).sqrt() as i32;
        while area % sq != 0 {
            sq -= 1;
        }
        vec![area / sq, sq]
    }
}

fn main() {}
