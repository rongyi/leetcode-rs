#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        match letters.binary_search(&target) {
            Ok(mut i) => {
                while i < letters.len() && letters[i] == target {
                    i += 1;
                }
                if i == letters.len() {
                    return letters[0];
                }
                letters[i]
            }
            Err(i) => {
                if i == letters.len() {
                    return letters[0];
                }
                letters[i]
            }
        }
    }
}

fn main() {
    let input = vec!['c', 'f', 'j'];
    let a = Solution::next_greatest_letter(input, 'd');
    println!("{}", a);
}
