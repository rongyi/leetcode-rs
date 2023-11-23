struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ret = 0;
        let mut prev_value = 0;

        for c in s.chars().rev() {
            let value = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };
            if value < prev_value {
                ret -= value;
            } else {
                ret += value;
            }
            prev_value = value;
        }
        ret
    }
}
