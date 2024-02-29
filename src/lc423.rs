
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut cnt = HashMap::new();
        for c in s.chars() {
            *cnt.entry(c).or_insert(0) += 1;
        }
        let mut digits = vec![0; 10];

        // z only in zero
        digits[0] = *cnt.get(&'z').unwrap_or(&0);
        // w only in two
        digits[2] = *cnt.get(&'w').unwrap_or(&0);
        // four
        digits[4] = *cnt.get(&'u').unwrap_or(&0);
        // six
        digits[6] = *cnt.get(&'x').unwrap_or(&0);
        // eight
        digits[8] = *cnt.get(&'g').unwrap_or(&0);

        digits[1] = (*cnt.get(&'o').unwrap_or(&0) - digits[0] - digits[2] - digits[4]).max(0); // 'o' only in one, zero, two, four
        digits[3] = (*cnt.get(&'h').unwrap_or(&0) - digits[8]).max(0); // 'h' only in three, eight
        digits[5] = (*cnt.get(&'f').unwrap_or(&0) - digits[4]).max(0); // 'f' only in five, four
        digits[7] = (*cnt.get(&'s').unwrap_or(&0) - digits[6]).max(0); // 's' only in seven, six
        digits[9] = (*cnt.get(&'i').unwrap_or(&0) - digits[5] - digits[6] - digits[8]).max(0); // 'i' only in nine, five, six, eight

        let mut ret = String::new();
        for (d, &count) in digits.iter().enumerate() {
            if count > 0 {
                ret += &(d as i32).to_string().repeat(count);
            }
        }

        ret
    }
}

fn main() {}
