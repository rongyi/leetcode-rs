struct Solution;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let sign = if num >= 0 { 1 } else { -1 };
        let mut num = num.abs();
        let mut ret = String::new();

        while num != 0 {
            let c = (num % 7).to_string();
            ret.push_str(&c);

            num /= 7;
        }
        if sign == -1 {
            ret.push('-');
        }

        ret.chars().rev().collect()
    }
}

fn main() {}
