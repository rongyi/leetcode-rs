struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut decode_table: HashMap<char, char> = HashMap::new();
        let mut cur: u8 = 0;
        for c in key.chars() {
            if c == ' ' {
                continue;
            }
            if !decode_table.contains_key(&c) {
                let to: char = ('a' as u8 + cur) as char;
                decode_table.insert(c, to);

                cur += 1;
            }
        }
        let mut out: String = String::new();
        for c in message.chars() {
            out.push(*decode_table.get(&c).unwrap_or(&' '));
        }

        out
    }
}

fn main() {}
