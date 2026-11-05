struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut ret = Vec::with_capacity(num_rows);

        for i in 0..num_rows {
            let mut row = Vec::with_capacity(i + 1);
            row.push(1);
            if i > 0 {
                let prev_row: &Vec<i32> = &ret[i - 1];
                for j in 0..prev_row.len() - 1 {
                    row.push(prev_row[j] + prev_row[j + 1]);
                }
                row.push(1);
            }
            ret.push(row);
        }

        ret
    }
}

