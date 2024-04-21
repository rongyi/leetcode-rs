#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_one_bit_character(mut bits: Vec<i32>) -> bool {
        bits.pop();
        let sz = bits.len();
        let mut i = 0;
        while i < sz {
            if bits[i] == 0 {
                i += 1;
            } else {
                if bits[i] == 1 {
                    if i + 1 >= sz {
                        return false;
                    }
                    i += 2;
                }
            }
        }
        true
    }
}

fn main() {}
