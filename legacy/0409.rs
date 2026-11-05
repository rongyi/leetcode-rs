struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut cnt: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let e = cnt.entry(c).or_insert(0);
            *e += 1;
        }
        let mut hash_odd = false;
        let mut acc = 0;
        for (&_k, &v) in cnt.iter() {
            if v % 2 == 1 {
                hash_odd = true;
            }
            acc += v / 2;
        }
        acc *= 2;

        if hash_odd {
            acc += 1;
        }

        acc
    }
}

fn main() {
    println!("{}", 10i32.pow(2));
}
