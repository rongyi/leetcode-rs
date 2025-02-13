#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut mask: HashMap<i32, i32> = HashMap::new();
        for rsv in reserved_seats.iter() {
            if let Some(&val) = mask.get(&rsv[0]) {
                mask.insert(rsv[0], val | (1 << rsv[1]));
            } else {
                mask.insert(rsv[0], 1 << rsv[1]);
            }
        }
        let mut ret = 0;
        for (&_k, &cur_mask) in mask.iter() {
            let mut cur_cnt = 0;

            if ((15 << 2) & cur_mask) == 0 {
                cur_cnt += 1;
            }
            if ((15 << 6) & cur_mask) == 0 {
                cur_cnt += 1;
            }
            if cur_cnt == 0 && ((15 << 4) & cur_mask) == 0 {
                cur_cnt += 1;
            }
            ret += cur_cnt;
        }

        ret + 2 * (n - mask.len() as i32)
    }
}

fn main() {}
