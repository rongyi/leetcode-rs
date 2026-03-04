struct Solution;

impl Solution {
    pub fn maximum_xor_product(a: i64, b: i64, n: i32) -> i32 {
        let mut ax = a;
        let mut bx = b;
        let m = 1_000_000_007;
        for i in (0..n).rev() {
            let cur = 1 << i;
            // same '1' or '0'
            if (ax & cur) == (bx & cur) {
                ax |= cur;
                bx |= cur;
            } else {
                // give this '1' to smaller so multiply can be larger
                if ax > bx {
                    ax &= !cur;
                    bx |= cur;
                } else {
                    ax |= cur;
                    bx &= !cur;
                }
            }
        }

        (((ax % m) * (bx % m)) % m) as _
    }
}

fn main() {}
