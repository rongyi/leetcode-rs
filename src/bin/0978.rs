
struct Solution;

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let sz = arr.len();
        if sz == 1 {
            return 1;
        }

        let mut max_len = 1;
        let mut current_len = 1;
        // 0: none, 1: increasing, -1: decreasing
        let mut trend = 0;

        for i in 1..sz {
            if arr[i - 1] < arr[i] {
                if trend == -1 {
                    current_len += 1;
                } else {
                    current_len = 2;
                }
                trend = 1;
            } else if arr[i - 1] > arr[i] {
                if trend == 1 {
                    current_len += 1;
                } else {
                    current_len = 2;
                }
                trend = -1;
            } else {
                current_len = 1;
                trend = 0;
            }
            max_len = max_len.max(current_len);
        }

        max_len
    }
}

fn main() {}
