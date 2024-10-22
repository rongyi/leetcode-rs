struct Solution;

impl Solution {
    pub fn bitwise_complement(mut n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut i = 0;
        let mut swap = 0;
        while n > 0 {
            //
            if (n & 1) == 0 {
                swap += 2i32.pow(i);
            }

            n >>= 1;
            i += 1;
        }
        swap
    }
}

fn main() {}
