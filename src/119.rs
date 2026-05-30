struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        // index start with 0
        let mut prev_row = vec![1];

        for i in 1..=row_index as usize {
            // total nums: i + 1;
            let mut cur_row = vec![1; i + 1];

            for i in 1..cur_row.len() - 1 {
                cur_row[i] = prev_row[i] + prev_row[i - 1];
            }

            prev_row = cur_row;
        }

        prev_row
    }
}

fn main() {}
