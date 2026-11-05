
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned: HashSet<i32> = banned.into_iter().collect();
        let mut ret = 0;
        let mut sum = 0;

        for i in 1..=n {
            if banned.contains(&i) {
                continue;
            }
            let check = sum + i;
            if check <= max_sum {
                sum = check;
                ret += 1;
            } else {
                break;
            }
        }

        ret
    }
}

fn main() {}
