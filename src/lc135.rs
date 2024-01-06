struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut distribute = vec![1; n];

        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                // include accumulate case
                distribute[i] = distribute[i - 1] + 1;
            }
        }

        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                distribute[i] = distribute[i].max(distribute[i + 1] + 1);
            }
        }

        distribute.into_iter().sum()
    }
}

fn main() {}
