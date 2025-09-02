struct Solution;

impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        // f(n) = f(n - 1) + f(n - 2)
        // with init f(0) = 1
        //           f(1) = 2
        let mut f0 = 1i64;
        let mut f1 = 2i64;

        for _i in 2..=n {
            (f1, f0) = ((f1 + f0) % 1_000_000_007, f1);
        }

        ((f1 * f1) % 1_000_000_007) as _
    }
}

fn main() {}
