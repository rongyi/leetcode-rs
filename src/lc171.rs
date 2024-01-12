struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut sum = 0;

        for c in column_title.chars() {
            let val = c as u8 - 'A' as u8 + 1;
            sum = sum * 26 + val as i32;
        }

        sum
    }
}

fn main() {}
