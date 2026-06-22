struct Solution;

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        while num > 9 {
            let mut cur = num;
            let mut next_num = 0;
            while cur != 0 {
                next_num += cur % 10;
                cur /= 10;
            }
            num = next_num;
        }

        num
    }
}

fn main() {}
