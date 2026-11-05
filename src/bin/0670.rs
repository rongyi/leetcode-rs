struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut ds: Vec<i32> = num
            .to_string()
            .chars()
            .into_iter()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        let mut max_idx = -1;
        let mut maxval = -1;
        let mut left_idx = -1;
        let mut swap_idx = -1;

        for i in (0..ds.len()).rev() {
            if ds[i] > maxval {
                maxval = ds[i];
                max_idx = i as i32;
                continue;
            }
            if ds[i] < maxval {
                left_idx = i as i32;
                swap_idx = max_idx;
            }
        }
        if left_idx == -1 {
            return num;
        }
        ds.swap(left_idx as usize, swap_idx as usize);

        ds.into_iter().fold(0, |acc, x| acc * 10 + x)
    }
}

fn main() {}
