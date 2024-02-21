impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut ret = 10;
        let mut c = 1;
        for i in 1..n.min(10) {
            c = c * (10 - i);
            ret = ret + 9 * c;
        }

        ret
    }
}

fn main() {}
