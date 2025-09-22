struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut nh = names
            .into_iter()
            .zip(heights.into_iter())
            .collect::<Vec<(String, i32)>>();
        nh.sort_by(|a, b| b.1.cmp(&a.1));

        nh.into_iter().map(|p| p.0).collect()
    }
}

fn main() {}
