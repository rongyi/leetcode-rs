struct Solution;

mod ai {
    impl Solution {
        pub fn gcd_of_strings(str1: String, str2: String) -> String {
            // Step 1: Check if a common pattern exists
            if format!("{str1}{str2}") != format!("{str2}{str1}") {
                return String::new();
            }

            // Step 2: Compute the GCD of string lengths using the Euclidean algorithm
            let gcd_len = Self::gcd(str1.len(), str2.len());

            // Step 3: Return the prefix of length gcd_len
            str1[..gcd_len].to_string()
        }

        fn gcd(mut a: usize, mut b: usize) -> usize {
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        }
    }
}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1 == str2 {
            return str1;
        }
        // assume str1 is longer one
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
