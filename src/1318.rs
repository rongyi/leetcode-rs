#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut flip_cnt = 0;
        for i in 0..31 {
            let expect = c & (1 << i);
            let op1 = a & (1 << i);
            let op2 = b & (1 << i);
            // perfect
            if expect == (op1 | op2) {
                continue;
            } else {
                // 1 as result, 0, 0 as two op
                // so we need just one
                if expect != 0 {
                    flip_cnt += 1;
                } else {
                    // 0 as result, flip 1 to 0 if op1 or op2 is 1
                    if op1 != 0 {
                        flip_cnt += 1;
                    }
                    if op2 != 0 {
                        flip_cnt += 1;
                    }
                }
            }
        }

        flip_cnt
    }
}

fn main() {}
