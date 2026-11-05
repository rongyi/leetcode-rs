#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut result = String::new();
        let command = command.as_bytes();
        let mut i = 0;

        while i < command.len() {
            if command[i] == b'G' {
                result.push('G');
                i += 1;
            } else if i + 1 < command.len() && command[i] == b'(' && command[i + 1] == b')' {
                result.push('o');
                i += 2;
            } else if i + 3 < command.len()
                && command[i] == b'('
                && command[i + 1] == b'a'
                && command[i + 2] == b'l'
                && command[i + 3] == b')'
            {
                result.push_str("al");
                i += 4;
            } else {
                i += 1;
            }
        }

        result
    }
}

fn main() {}
