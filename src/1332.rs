#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        // take it easy, this is a easy problem, you dont need to think too much,
        // just contain 'a' and 'b' means we only need at most 2 delete action
        // one for 'a' and one for all 'b'
        // so if s is already a parlindrom this is one time delete action
        let s = s.as_bytes();
        let sz = s.len();
        let mut i = 0;
        let mut j = sz - 1;
        let mut is_parlindrom = true;
        while i < j {
            if s[i] != s[j] {
                is_parlindrom = false;
                break;
            }
            i += 1;
            j -= 1;
        }
        if is_parlindrom {
            1
        } else {
            2
        }
    }
}

fn main() {}
