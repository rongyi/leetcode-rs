struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let hlen = haystack.len();
        let nlen = needle.len();

        if hlen < nlen {
            return -1;
        }

        for i in 0..=(hlen - nlen) {
            if &haystack[i..(i + nlen)] == &needle {
                return i as i32;
            }
        }
        -1
    }
}
