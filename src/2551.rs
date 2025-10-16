struct Solution;

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let sz = weights.len();
        if sz == k as usize || k == 1 {
            return 0;
        }
        let mut max_score = 0i64;
        let mut min_score = 0i64;
        let mut score: Vec<i64> = vec![0; sz - 1];

        for i in 0..sz - 1 {
            score[i] = (weights[i] + weights[i + 1]) as i64;
        }
        score.sort_unstable();
        for i in 0..k as usize - 1 {
            max_score += score[sz - 2 - i];
            min_score += score[i];
        }

        max_score - min_score
    }
}

fn main() {}
