struct Solution;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut ret = 0;
        let mut n = n as i64;
        while n != 1 {
            if n % 2 == 1 {
                ret += 1;
                if n & 3 == 3 && n > 3 {
                    n += 1;
                }
            }
            n >>= 1;
            ret += 1;
        }

        ret
    }
}

fn main() {}
