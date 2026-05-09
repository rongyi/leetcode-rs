struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let m = haystack.len();
        let n = needle.len();
        if m < n {
            return -1;
        }
        for i in 0..=m - n {
            if &haystack[i..i + n] == needle {
                return i as _;
            }
        }

        -1
    }
}

fn main() {}
