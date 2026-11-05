#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut counts = [0; 26];

        for word in words.iter() {
            for c in word.chars() {
                counts[(c as u8 - b'a') as usize] += 1;
            }
        }

        let n = words.len();

        for count in counts.iter() {
            if *count % n != 0 {
                return false;
            }
        }

        true
    }
}

fn main() {}
