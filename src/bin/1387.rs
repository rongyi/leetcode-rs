#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        // do as they told
        let mut memo: HashMap<i32, i32> = HashMap::new();
        fn steps(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if n == 1 {
                return 0;
            }
            if memo.contains_key(&n) {
                return memo[&n];
            }
            let ret;
            if n & 1 == 0 {
                ret = 1 + steps(n / 2, memo);
            } else {
                ret = 1 + steps(n * 3 + 1, memo);
            }
            memo.insert(n, ret);

            ret
        }
        let mut arr: Vec<_> = (lo..=hi).collect();
        arr.sort_by(|&a, &b| {
            steps(a, &mut memo)
                .cmp(&steps(b, &mut memo))
                .then(a.cmp(&b))
        });

        arr[k as usize - 1]
    }
}

fn main() {}
