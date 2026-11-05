struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut out = vec![vec![1]];
        for r in 2..=num_rows as usize {
            let prev_row = &out[r - 2]; // 1 index so another 1
            let mut new_row = vec![1; r];

            for i in 1..r - 1 {
                new_row[i] = prev_row[i] + prev_row[i - 1];
            }

            out.push(new_row);
        }

        out
    }
}

fn main() {}
