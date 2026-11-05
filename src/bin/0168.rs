struct Solution;

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut ret = String::new();
        while column_number > 0 {
            column_number -= 1;

            let c = 'A' as u8 + (column_number % 26) as u8;
            ret.push(c as char);

            column_number /= 26;
        }
        ret.chars().rev().collect()
    }
}

fn main() {
    Solution::convert_to_title(701);
}
