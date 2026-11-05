struct Solution;

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut k = num;
        let mut ret = 0;
        while k > 0 {
            let cur = k % 10;
            if num % cur == 0 {
                ret += 1;
            }
            k /= 10;
        }
        ret
    }
}

fn main() {}
