#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
        let mut ret = 0;
        let sz = row.len() / 2;

        for i in 0..sz {
            let one = row[2 * i];
            let another = Self::my_couple(one);
            // already sit together
            if another == row[2 * i + 1] {
                continue;
            }
            ret += 1;

            for j in 2 * (i + 1)..2 * sz {
                if row[j] == another {
                    row[j] = row[2 * i + 1];
                    break;
                }
            }
        }

        ret
    }

    fn my_couple(i: i32) -> i32 {
        if i & 1 == 1 {
            i - 1
        } else {
            i + 1
        }
    }
}

fn main() {}
