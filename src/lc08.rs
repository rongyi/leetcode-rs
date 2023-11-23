struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut it = s.chars().peekable();
        // ignore leading spaces
        while let Some(&' ') = it.peek() {
            it.next();
        }
        let mut sign = 1;
        let mut ret: i64 = 0;
        // check sign;
        if let Some(c) = it.next() {
            match c {
                '+' => {}
                '-' => {
                    sign = -1;
                }
                '0'..='9' => {
                    ret = c.to_digit(10).unwrap() as i64;
                }
                _ => {
                    return 0;
                }
            }
        }
        for c in it {
            if let Some(d) = c.to_digit(10) {
                ret = ret * 10 + d as i64;
                if sign == 1 && ret > i32::MAX as i64 {
                    return i32::MAX;
                }
                if sign == -1 && ret * sign < i32::MIN as i64 {
                    return i32::MIN;
                }
            } else {
                break;
            }
        }

        (ret * sign) as i32
    }
}
