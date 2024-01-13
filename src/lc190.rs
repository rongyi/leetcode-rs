struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut ret = 0;

        for i in 0..32 {
            let val = (x >> i) & 1;
            ret |= val << (32 - i - 1);
        }

        ret
    }
}

fn main() {}
