#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut mask: HashMap<i32, Vec<i32>> = HashMap::new();
        for rsv in reserved_seats.iter() {
            mask.entry(rsv[0]).or_default().push(rsv[1]);
        }

        let mut ret = 0;
        for (&_k, cur_mask) in mask.iter() {
            let mut blocks = vec![false; 3];

            for &pos in cur_mask.iter() {
                if pos >= 2 && pos <= 5 {
                    blocks[0] = true;
                }
                if pos >= 6 && pos <= 9 {
                    blocks[2] = true;
                }
                if pos >= 4 && pos <= 7 {
                    blocks[1] = true;
                }
            }

            if !blocks[0] && !blocks[2] {
                ret += 2;
            } else if blocks.iter().any(|&t| !t) {
                ret += 1
            }
        }

        ret + 2 * (n - mask.len() as i32)
    }
}

fn main() {}
