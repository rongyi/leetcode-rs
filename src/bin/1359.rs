#![allow(dead_code)]

struct Solution;

impl Solution {
    // 假设 n - 1pair已经排好现在来排最后的两个p和d
    // p可以插入的位置有 2 * (n - 1) + 1 ==> 2n - 1
    // d可以插入的位置由 (2*(n - 1) + 1) + 1 ==> 2n
    // 然后p要在前，除以2少一半
    pub fn count_orders(n: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let mut ret: i64 = 1;

        for i in 1..=n {
            let i = i as i64;
            ret = ret * (i * 2 - 1) * i % MOD;
        }

        ret as i32
    }
}

fn main() {}
