#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        let start: Vec<char> = start.chars().collect();
        let end: Vec<char> = end.chars().collect();

        let clean_start: Vec<char> = start
            .iter()
            .filter(|&&c| c != 'X')
            .map(|c| c.to_owned())
            .collect();
        let clean_end: Vec<char> = end
            .iter()
            .filter(|&&c| c != 'X')
            .map(|c| c.to_owned())
            .collect();
        if clean_start != clean_end {
            return false;
        }

        let sz = start.len();
        let mut i = 0;
        let mut j = 0;
        while i < sz && j < sz {
            while i < sz && start[i] == 'X' {
                i += 1;
            }
            while j < sz && end[j] == 'X' {
                j += 1;
            }
            if i == sz && j == sz {
                return true;
            }
            if i == sz || j == sz || start[i] != end[j] {
                return false;
            }
            // L go left
            if start[i] == 'L' && i < j {
                return false;
            }

            // R go right
            if start[i] == 'R' && i > j {
                return false;
            }
            i += 1;
            j += 1;
        }

        true
    }
}

fn main() {
    Solution::can_transform("RXR".to_string(), "XXR".to_string());
}
