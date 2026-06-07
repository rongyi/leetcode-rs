struct Solution;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut reverse = 0;

        for i in 0..32 {
            let cur = (n >> i) & 1;
            if cur != 0 {
                reverse |= 1 << (32 - i - 1);
            }
        }

        reverse
    }
}
fn main() {}
