#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut cur_sz = 0;
        let mut line_number = 0;
        let max_line = 100;

        for c in s.chars() {
            let idx = (c as u8 - 'a' as u8) as usize;
            cur_sz += widths[idx];
            // too long, create a fresh line
            if cur_sz > max_line {
                line_number += 1;
                cur_sz = widths[idx];
            }
        }

        vec![line_number + 1, cur_sz]
    }
}

fn main() {}
