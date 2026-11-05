struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut acc = 0;

        while x != 0 {
            let cur = x % 10;
            x /= 10;

            if acc > i32::MAX / 10 || (acc == i32::MAX / 10 && cur > i32::MAX % 10) {
                return 0;
            }

            if acc < i32::MIN / 10 || (acc == i32::MIN / 10 && cur < i32::MIN % 10) {
                return 0;
            }

            acc = acc * 10 + cur
        }

        acc
    }
}

fn main() {
    println!("{}", i32::MIN % 10);
    println!("{}", -32 % 10);
}
