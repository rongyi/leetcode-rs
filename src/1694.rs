#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let digits = number
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<Vec<char>>();

        let n = digits.len();
        let mut result = String::new();
        let mut i = 0;

        while i < n {
            // If there are 4 digits left, group as 2-2
            if n - i == 4 {
                result.push(digits[i]);
                result.push(digits[i + 1]);
                result.push('-');
                result.push(digits[i + 2]);
                result.push(digits[i + 3]);
                break;
            }
            // If there are 2 digits left, add them as the final group
            else if n - i == 2 {
                result.push(digits[i]);
                result.push(digits[i + 1]);
                break;
            }
            // If there are 3 digits left, add them as the final group
            else if n - i == 3 {
                result.push(digits[i]);
                result.push(digits[i + 1]);
                result.push(digits[i + 2]);
                break;
            }
            // Otherwise, add 3 digits and a dash
            else {
                result.push(digits[i]);
                result.push(digits[i + 1]);
                result.push(digits[i + 2]);
                result.push('-');
                i += 3;
            }
        }

        result
    }
}

fn main() {}
