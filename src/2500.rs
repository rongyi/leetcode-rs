struct Solution;

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        for r in grid.iter_mut() {
            r.sort_unstable();
        }
        let mut sum = 0;

        for _ in 0..grid[0].len() {
            let mut val = i32::MIN;
            for r in grid.iter_mut() {
                val = val.max(r.pop().unwrap());
            }
            sum += val;
        }

        sum
    }
}

fn main() {}
