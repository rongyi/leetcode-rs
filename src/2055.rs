pub struct Solution;

impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let s: Vec<char> = s.chars().collect();
        let sz = queries.len();
        let mut ret = vec![0; sz];

        let mut star_acc = 0;
        let mut prefix_sum = vec![0; s.len() + 1];
        let mut right_most = vec![-1; s.len()];
        let mut left_most = vec![-1; s.len()];

        let mut left_bar_index = -1;
        for (i, &c) in s.iter().enumerate() {
            if c == '*' {
                star_acc += 1;
                left_most[i] = left_bar_index;
            } else {
                left_bar_index = i as i32;
                left_most[i] = i as i32;
            }
            prefix_sum[i + 1] = star_acc;
        }
        let mut right_bar_index = -1;
        for (i, &c) in s.iter().enumerate().rev() {
            if c == '*' {
                right_most[i] = right_bar_index;
            } else {
                right_bar_index = i as i32;
                right_most[i] = i as i32;
            }
        }

        for (i, q) in queries.iter().enumerate() {
            let (left, right) = (q[0], q[1]);
            // either at '*' or '|' it all count zero
            if left == right {
                ret[i] = 0;
                continue;
            }
            let left_bar = right_most[left as usize];
            let right_bar = left_most[right as usize];
            if left_bar == -1 || right_bar == -1 {
                ret[i] = 0;
                continue;
            }
            if left_bar >= right_bar {
                ret[i] = 0;
                continue;
            }
            let sum = prefix_sum[right_bar as usize + 1] - prefix_sum[left_bar as usize];
            ret[i] = sum;
        }

        ret
    }
}

fn main() {}
