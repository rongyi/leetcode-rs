#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let sum: i32 = card_points.iter().sum();
        if k >= card_points.len() as i32 {
            return sum;
        }
        let win_sz = card_points.len() - k as usize;
        let mut win_sum: i32 = card_points.iter().take(win_sz).sum();
        let mut min_win_sum = win_sum;
        for i in win_sz..card_points.len() {
            win_sum += card_points[i] - card_points[i - win_sz];
            min_win_sum = min_win_sum.min(win_sum);
        }
        sum - min_win_sum
    }
}

fn main() {}
