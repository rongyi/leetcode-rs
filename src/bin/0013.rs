struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.as_bytes();
        let mut val = 0;
        let sz = s.len();
        let mut i = 0;

        while i < sz {
            if s[i] == b'I' {
                if i + 1 < sz && s[i + 1] == b'V' {
                    val += 4;
                    i += 1;
                } else if i + 1 < sz && s[i + 1] == b'X' {
                    val += 9;
                    i += 1;
                } else {
                    val += 1;
                }
            } else if s[i] == b'X' {
                if i + 1 < sz && s[i + 1] == b'L' {
                    val += 40;
                    i += 1;
                } else if i + 1 < sz && s[i + 1] == b'C' {
                    val += 90;
                    i += 1;
                } else {
                    val += 10;
                }
            } else if s[i] == b'C' {
                if i + 1 < sz && s[i + 1] == b'D' {
                    val += 400;
                    i += 1;
                } else if i + 1 < sz && s[i + 1] == b'M' {
                    val += 900;
                    i += 1;
                } else {
                    val += 100;
                }
            } else if s[i] == b'V' {
                val += 5;
            } else if s[i] == b'X' {
                val += 10;
            } else if s[i] == b'M' {
                val += 1000;
            } else if s[i] == b'L' {
                val += 50;
            } else if s[i] == b'D' {
                val += 500;
            }

            i += 1;
        }

        val
    }
}

fn main() {}
