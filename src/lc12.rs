struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let roman_numerals = vec![
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];
        let mut ret = String::new();
        let mut num = num;

        for &(symbol, value) in &roman_numerals {
            while num >= value {
                ret.push_str(symbol);
                num -= value;
            }
        }

        ret
    }
}
