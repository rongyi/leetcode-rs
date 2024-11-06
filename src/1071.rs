struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1 == str2 {
            return str1;
        }
        if str1.len() < str2.len() {
            return Self::gcd_of_strings(str2, str1);
        }
        if str2.is_empty() {
            return str1;
        }

        if str1.starts_with(&str2) {
            let sz = str2.len();
            let mut i = sz;
            while i < str1.len() && i + sz <= str1.len() {
                let cur: String = str1[i..i + sz].chars().collect();
                if cur == str2 {
                    i += sz;
                } else {
                    break;
                }
            }

            let d: String = str1[i..].chars().collect();
            return Self::gcd_of_strings(d, str2);
        }

        return "".to_string();
    }
}

fn main() {
    let a = "hello ".to_string();
    let b = a[..3].chars().collect::<String>();
    println!("{}", b);
}
