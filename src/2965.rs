struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let total = n * n;
        let mut count = vec![0; total + 1];
        let mut repeated = 0;
        let mut missing = 0;

        for row in grid {
            for num in row {
                count[num as usize] += 1;
            }
        }

        for i in 1..=total {
            if count[i] == 2 {
                repeated = i as i32;
            }
            if count[i] == 0 {
                missing = i as i32;
            }
        }

        vec![repeated, missing]
    }
}

fn main() {}
