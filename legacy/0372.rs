struct Solution;

const MOD: i32 = 1337;

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        fn pow(a: i32, b: i32) -> i32 {
            if b == 0 {
                1
            } else {
                let mut ret = 1;
                let mut base = a % MOD;
                let mut exponent = b;

                while exponent > 0 {
                    if exponent % 2 == 1 {
                        ret = (ret * base) % MOD;
                    }
                    base = (base * base) % MOD;
                    exponent /= 2;
                }

                ret
            }
        }
        let mut ret = 1;
        for num in b {
            ret = pow(ret, 10) * pow(a, num) % MOD;
        }
        ret
    }
}

fn main() {}
