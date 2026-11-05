use rand::{thread_rng, Rng};

struct Solution {
    sum: Vec<i32>,
    total: i32,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut sum: Vec<i32> = vec![0; w.len()];
        sum[0] = w[0];
        for i in 1..w.len() {
            sum[i] = sum[i - 1] + w[i];
        }
        let total = sum.last().unwrap().to_owned();

        Self { sum, total }
    }

    fn pick_index(&self) -> i32 {
        let mut rng = thread_rng();
        let val = rng.gen_range(0..self.total);
        // the upper_bound
        let mut left = 0;
        let mut right = self.sum.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            if val >= self.sum[mid] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}

fn main() {}
