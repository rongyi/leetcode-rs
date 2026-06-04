struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }
        let mut ret: String = String::new();

        if (numerator < 0) ^ (denominator < 0) {
            ret.push('-');
        }
        let a = (numerator as i64).abs();
        let b = (denominator as i64).abs();
        let integer_part = a / b;
        ret.push_str(&integer_part.to_string());
        let mut rem = a % b;
        if rem == 0 {
            return ret;
        }
        // decimal part
        ret.push('.');
        let mut rem_cache = HashMap::new();

        while rem != 0 {
            if let Some(&pos) = rem_cache.get(&rem) {
                ret.insert(pos, '(');
                ret.push(')');
                break;
            }
            rem_cache.insert(rem, ret.len());

            rem *= 10;
            ret.push_str(&(rem / b).to_string());
            rem %= b;
        }

        ret
    }
}

fn main() {}
