#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn num_friend_requests(mut ages: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        for &age in ages.iter() {
            *cnt.entry(age).or_insert(0) += 1;
        }
        let mut ret = 0;
        for (&k1, &v1) in cnt.iter() {
            for (&k2, &v2) in cnt.iter() {
                if Self::request(k1, k2) {
                    if k1 == k2 {
                        ret += v1 * (v2 - 1);
                    } else {
                        ret += v1 * v2;
                    }
                }
            }
        }

        ret
    }
    fn request(a: i32, b: i32) -> bool {
        !((b <= a / 2 + 7) || b > a || (b > 100) && a < 100)
    }
}

fn main() {}
