struct Solution;

impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = 1e5 as i32;
        while left < right {
            let mid = left + (right - left) / 2;
            let expected_count: i32 = quantities.iter().map(|&x| (x + mid - 1) / mid).sum();
            if expected_count > n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

fn main() {}
