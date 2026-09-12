struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        // 提前剪枝：只含 1 的数末尾一定是 1，不可能被 2 或 5 整除
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }

        let mut rem = 1 % k; // n_1 mod k

        for length in 1..=k {
            if rem == 0 {
                return length;
            }
            // 递推下一个全 1 数的余数
            rem = (rem * 10 + 1) % k;
        }

        -1
    }
}

fn main() {}
