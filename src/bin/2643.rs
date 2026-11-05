struct Solution;

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let ones: Vec<i32> = mat
            .iter()
            .map(|l| l.iter().filter(|x| **x == 1).count() as _)
            .collect();
        let mut max_one = 0;
        let mut idx = 0;
        for i in (0..ones.len()).rev() {
            if ones[i] >= max_one {
                max_one = ones[i];
                idx = i;
            }
        }
        vec![idx as i32, ones[idx]]
    }
}

fn main() {}
