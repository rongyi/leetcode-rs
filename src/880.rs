struct Solution;

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut sz: i64 = 0;
        let mut k = k as i64;

        for c in s.chars() {
            if c.is_ascii_digit() {
                sz *= c.to_digit(10).unwrap() as i64;
            } else {
                sz += 1;
            }
        }
        for c in s.chars().rev() {
            k %= sz;
            if k == 0 && c.is_ascii_alphabetic() {
                return c.to_string();
            }
            if c.is_ascii_digit() {
                sz /= c.to_digit(10).unwrap() as i64;
            } else {
                sz -= 1;
            }
        }

        unreachable!()
    }
}

fn main() {}
