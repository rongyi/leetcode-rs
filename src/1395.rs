#![allow(dead_code)]

struct Solution;

impl Solution {
    // For each soldier, count how many soldiers on the left and right have less
    // and greater ratings.
    // This soldier can form less[left] * greater[right] + greater[left] *
    // less[right] teams.
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let sz = rating.len();
        let mut ret = 0;
        for i in 1..sz - 1 {
            // 0: left, 1: right
            let mut less = vec![0, 0];
            let mut great = vec![0, 0];
            for j in 0..sz {
                if rating[j] > rating[i] {
                    if i < j {
                        great[0] += 1;
                    } else {
                        great[1] += 1;
                    }
                }
                if rating[j] < rating[i] {
                    if i < j {
                        less[0] += 1;
                    } else {
                        less[1] += 1;
                    }
                }
            }
            ret += less[0] * great[1] + less[1] * great[0];
        }
        ret
    }
}

fn main() {}
