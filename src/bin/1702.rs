#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let mut binary: Vec<char> = binary.chars().collect();
        let mut left = 0;
        let mut right = 1;
        while right < binary.len() {
            if binary[left] == '0' && binary[right] == '0' {
                binary[left] = '1';
                binary[right] = '1';
                binary[left + 1] = '0';
                left += 1;
                right += 1;
            } else if binary[left] == '1' {
                left += 1;
                right = left + 1;
            } else {
                right += 1;
            }
        }

        binary.into_iter().collect()
    }
}

fn main() {}
