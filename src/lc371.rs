struct Solution;

impl Solution {
    // do not use + or -
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut carry = 0;
        let mut ret = 0;
        let mut mask = 1u32;
        while mask != 0 {
            // use ^ to mimic add
            ret |= (a ^ b ^ carry) & mask as i32;
            // carry for case 1 + 1 or (1 + 0) / (0 + 1) with carry == 1
            carry = ((a & b) | ((a ^ b) & carry)) << 1;

            mask = mask << 1;
        }

        ret
    }
}

fn main() {}
