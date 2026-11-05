struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut row = Vec::with_capacity(row_index + 1);
        for i in 0..=row_index {
            row.push(1);
            if i > 1 {
                for j in (1..i).rev() {
                    row[j] += row[j - 1];
                }
            }
        }

        row
    }
}

