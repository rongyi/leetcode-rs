struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_number = column_number;
        let mut ret = String::new();

        while column_number > 0 {
            let r = (column_number - 1) % 26;
            ret.push(('A' as u8 + r as u8) as char);
            column_number = (column_number - 1) / 26;
        }

        ret.chars().rev().collect()
    }
}

fn main() {}
