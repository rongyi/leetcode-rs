struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }
        let mut ret = String::new();
        if (numerator < 0) ^ (denominator < 0) {
            ret.push('-');
        }
        // cast to i64 first then take abs in case overflow
        let numerator = (numerator as i64).abs();
        let denominator = (denominator as i64).abs();

        let val = numerator / denominator;
        ret.push_str(&val.to_string());

        let mut r = numerator % denominator;
        if r == 0 {
            return ret;
        }

        ret.push('.');
        let mut index = 0;
        let mut visit: HashMap<i64, usize> = HashMap::new();
        let mut fraction = String::new();
        while r != 0 {
            if visit.contains_key(&r) {
                let i = visit[&r];
                fraction.insert(i, '(');
                fraction.push(')');
                break;
            }
            visit.insert(r, index);

            r *= 10;
            fraction.push_str(&(r / denominator).to_string());
            r %= denominator;

            index += 1;
        }

        ret.push_str(&fraction);

        ret
    }
}

fn main() {
    let s = "1.0.11".to_string();
    for p in s.split(".") {
        println!("{p}");
    }
}
