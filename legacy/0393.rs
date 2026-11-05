struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let data: Vec<u8> = data.iter().map(|x| *x as u8 & 0xff).collect();
        let n = data.len();
        let mut i = 0;
        while i < n {
            // single valid
            if Self::iszero(data[i]) {
                i += 1;
                continue;
            }
            let one_len = Self::checkone(data[i]);
            if one_len < 2 || one_len > 4 || one_len > n - i {
                return false;
            }
            for j in i + 1..i + one_len {
                if (data[j] & 0xc0) != 0x80 {
                    return false;
                }
            }

            i += one_len;
        }

        true
    }

    fn iszero(i: u8) -> bool {
        (i & 0x80) == 0x00
    }
    fn checkone(i: u8) -> usize {
        let mut ret = 0;
        for j in (0..8).rev() {
            if ((1 << j) & i) != 0 {
                ret += 1;
            } else {
                break;
            }
        }
        ret
    }
}

fn main() {}
