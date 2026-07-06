struct Solution;

impl Solution {
    /// LeetCode 393: UTF-8 Validation
    ///
    /// Validates that `data` is a correctly encoded UTF-8 byte sequence.
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let bytes: Vec<u8> = data.iter().map(|&x| x as u8).collect();
        let sz = bytes.len();
        let mut i = 0;

        while i < sz {
            let one_cnt = Self::count_leading_ones(bytes[i]);
            if one_cnt == 0 {
                i += 1;
                continue;
            }
            if one_cnt < 2 || one_cnt > 4 {
                return false;
            }
            let need_len = one_cnt - 1;
            // too short
            if i + need_len >= sz {
                return false;
            }
            for j in i + 1..=i + need_len {
                let follow_one_cnt = Self::count_leading_ones(bytes[j]);
                if follow_one_cnt != 1 {
                    return false;
                }
            }

            i += one_cnt;
        }

        true
    }

    /// Counts leading 1-bits (e.g., 110xxxxx → 2, 0xxxxxxx → 0).
    fn count_leading_ones(byte: u8) -> usize {
        let mut count = 0;
        for shift in (0..8).rev() {
            if (byte >> shift) & 1 == 1 {
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}

fn main() {}
